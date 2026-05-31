//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 955/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk955<F: Float>(t1354: F, t3283: F, t1364: F, t3619: F, t3823: F, t3830: F, t423: F, t12873: F, t3831: F, t13125: F, t1349: F, t14093: F, t14116: F, t14118: F, t14120: F, t14122: F, t14126: F, t14129: F, t3819: F, t417: F, t451: F) -> F {
    let t14132 = t1354 * t3283;
    let t14133 = t14132 * t1364;
    let t14136 = t3823 * t3619;
    let t14140 = F::cast_from(1.0_f64) / t3830 / t423;
    let t14141 = t14140 * t12873;
    let t14145 = t3831 * t1364 * t3619;
    let t14149 = -F::cast_from(0.14055920378328537299e-1_f64) * t14116 - F::cast_from(0.28111840756657074597e-1_f64) * t14118 - F::cast_from(0.42167761134985611897e-1_f64) * t14120 - F::cast_from(0.14055920378328537299e-1_f64) * t14093 * t14122 - F::cast_from(0.28111840756657074597e-1_f64) * t3819 * t14126 + F::cast_from(0.14055920378328537299e-1_f64) * t3819 * t14129 + F::cast_from(0.14055920378328537299e-1_f64) * t1349 * t14133 + F::cast_from(0.14055920378328537299e-1_f64) * t1349 * t14136 - F::cast_from(0.56223681513314149196e-1_f64) * t417 * t14141 + F::cast_from(0.42167761134985611897e-1_f64) * t417 * t14145 - t13125 * t451;
    t14149
}
