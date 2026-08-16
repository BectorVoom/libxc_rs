//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 550/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk550<F: Float>(t169: F, t3153: F, t3157: F, t1012: F, t561: F, t182: F, t1667: F, t3017: F, t1043: F, t1019: F, t3080: F, t1040: F, t3121: F) -> (F, F, F, F, F, F, F) {
    let t3158 = t169 * t3153 * t3157;
    let t3160 = t561 * t1012;
    let t3161 = t3160 * t182;
    let t3163 = t3017 * t1667;
    let t3164 = t1043 * t3163;
    let t3166 = t3080 * t1019;
    let t3168 = t3121 * t1040;
    (t3158, t3160, t3161, t3163, t3164, t3166, t3168)
}
