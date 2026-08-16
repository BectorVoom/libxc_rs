//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1473;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1474;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta265(t10115: f64, t557: f64, t1429: f64, t9292: f64, t3964: f64, t4096: f64, t9285: f64, t1398: f64, t215: f64, t268: f64, t543: f64, t4101: f64, t2453: f64, t4100: f64, t281: f64, t68: f64, t562: f64, t2435: f64, t3903: f64, t1445: f64, t3895: f64, t2439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10117, t10126, t10129, t10136, t10137) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1473(t10115, t557, t1429, t9292, t3964, t4096, t9285, t1398, t215, t268, t543, t4101);
        let t10139 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1474(t2453, t4100);
        let (t10142, t10143, t10157, t10160, t10162, t10163) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1475(t1398, t281, t543, t68, t10139, t10115, t562, t2435, t3903, t1445, t3895, t2439);
    (t10117, t10126, t10129, t10136, t10137, t10139, t10142, t10143, t10157, t10160, t10162, t10163)
}
