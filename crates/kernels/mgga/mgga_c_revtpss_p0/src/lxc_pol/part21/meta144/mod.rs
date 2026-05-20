//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk927;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk928;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk929;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta144<F: Float>(t3390: F, t3391: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t1132: F, t406: F, t1139: F, t281: F, t2902: F, t414: F, t1146: F, t698: F, t1224: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3392, t3394, t3399) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk927::<F>(t3390, t3391, t3356, t3358, t3365, t3370, t3374);
        let (t3400, t3402, t3407) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk928::<F>(t1132, t3399, t3356, t406);
        let (t3408, t3410, t3413, t3414, t3415) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk929::<F>(t3391, t3407, t1139, t3399, t281, t2902, t414, t1146, t698);
        let t3417 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk930::<F>(t1224, t240);
    (t3392, t3394, t3399, t3400, t3402, t3407, t3408, t3410, t3413, t3414, t3415, t3417)
}
