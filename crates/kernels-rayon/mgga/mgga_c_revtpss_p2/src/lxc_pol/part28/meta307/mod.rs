//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1307;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1308;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta307(t1398: f64, t215: f64, t268: f64, t543: f64, t4101: f64, t2453: f64, t4100: f64, t281: f64, t68: f64, t1357: f64, t4078: f64, t689: f64, t1445: f64, t3899: f64, t10115: f64, t562: f64, t2435: f64, t3903: f64, t3895: f64, t2439: f64, t1420: f64, t3908: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10137, t10139, t10143, t10151) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1307(t1398, t215, t268, t543, t4101, t2453, t4100, t281, t68, t1357, t4078, t689);
        let (t10154, t10157, t10160, t10163, t10166) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1308(t1445, t3899, t689, t10115, t562, t2435, t3903, t3895, t2439, t1420, t2453, t3908);
    (t10137, t10139, t10143, t10151, t10154, t10157, t10160, t10163, t10166)
}
