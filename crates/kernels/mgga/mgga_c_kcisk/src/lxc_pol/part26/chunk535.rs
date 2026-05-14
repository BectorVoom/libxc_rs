//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 535/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk535<F: Float>(t10: F, t1337: F, t1229: F, t1233: F, t1232: F, t357: F, t346: F, t344: F, t347: F, t4007: F, t313: F, t353: F, t964: F, t1163: F, t1248: F, t3979: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4013 = t10 * t1337;
    let t4026 = t1229 * t1233;
    let t4029 = t1232 * t357;
    let t4030 = 1.0 / t4029;
    let t4031 = t346 * t4030;
    let t4037 = 1.0 / t347 / t344;
    let t4041 = 4.0 / 9.0 * t4007;
    let t4049 = 0.39862222222222222223e0 * t4007;
    let t4054 = 1.0/f64::sqrt(t344);
    let t4060 = t353 * t964 * t313;
    let t4061 = 0.27385555555555555555e0 * t4060;
    let t4063 = t1248 * t3979 * t1163;
    (t4013, t4026, t4030, t4031, t4037, t4041, t4049, t4054, t4060, t4061, t4063)
}
