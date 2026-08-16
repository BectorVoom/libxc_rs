//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1473;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1474;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta265<F: Float>(t10115: F, t557: F, t1429: F, t9292: F, t3964: F, t4096: F, t9285: F, t1398: F, t215: F, t268: F, t543: F, t4101: F, t2453: F, t4100: F, t281: F, t68: F, t562: F, t2435: F, t3903: F, t1445: F, t3895: F, t2439: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10117, t10126, t10129, t10136, t10137) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1473::<F>(t10115, t557, t1429, t9292, t3964, t4096, t9285, t1398, t215, t268, t543, t4101);
        let t10139 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1474::<F>(t2453, t4100);
        let (t10142, t10143, t10157, t10160, t10162, t10163) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1475::<F>(t1398, t281, t543, t68, t10139, t10115, t562, t2435, t3903, t1445, t3895, t2439);
    (t10117, t10126, t10129, t10136, t10137, t10139, t10142, t10143, t10157, t10160, t10162, t10163)
}
