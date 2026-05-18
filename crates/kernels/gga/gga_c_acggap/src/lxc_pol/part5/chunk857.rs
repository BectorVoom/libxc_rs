//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 857/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk857<F: Float>(t12200: F, t464: F, t181: F, t862: F, t322: F, t3888: F, t448: F, t3868: F, t3915: F, t1220: F, t1221: F, t316: F, t879: F) -> (F, F, F, F, F) {
    let t12201 = t12200 * t464;
    let t12203 = t862 * t181;
    let t12206 = t12203 * t448 * t322 * t3888;
    let t12208 = t3868 * t3915;
    let t12212 = t316 * t1220 * t879 * t1221;
    (t12201, t12203, t12206, t12208, t12212)
}
