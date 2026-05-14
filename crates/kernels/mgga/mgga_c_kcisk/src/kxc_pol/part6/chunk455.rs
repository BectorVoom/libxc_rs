//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 455/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk455<F: Float>(t4007: F, t344: F, t313: F, t353: F, t964: F, t1311: F, t24: F, t1232: F, t346: F, t360: F, t1265: F, t370: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4041 = 4.0 / 9.0 * t4007;
    let t4049 = 0.39862222222222222223e0 * t4007;
    let t4054 = 1.0/f64::sqrt(t344);
    let t4060 = t353 * t964 * t313;
    let t4061 = 0.27385555555555555555e0 * t4060;
    let t4065 = t24 * t1311;
    let t4079 = t1232 * t1232;
    let t4080 = 1.0 / t4079;
    let t4081 = t346 * t4080;
    let t4082 = t360 * t360;
    let t4083 = 1.0 / t4082;
    let t4087 = 0.12361111111111111111e-1 * t4007;
    let t4099 = t1265 * t370;
    let t4100 = 1.0 / t4099;
    let t4108 = 0.40256666666666666667e0 * t4007;
    let t4115 = 0.27595e0 * t4060;
    let t4125 = t1265 * t1265;
    let t4126 = 1.0 / t4125;
    (t4041, t4049, t4054, t4061, t4065, t4079, t4080, t4081, t4082, t4083, t4087, t4100, t4108, t4115, t4125, t4126)
}
