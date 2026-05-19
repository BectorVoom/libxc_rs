//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 472/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk472<F: Float>(t4029: F, t346: F, t344: F, t347: F, t4007: F, t313: F, t353: F, t964: F, t1311: F, t24: F, t1232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4030 = F::new(1.0) / t4029;
    let t4031 = t346 * t4030;
    let t4037 = F::new(1.0) / t347 / t344;
    let t4041 = F::new(4.0) / F::new(9.0) * t4007;
    let t4049 = F::cast_from(0.39862222222222222223e0_f64) * t4007;
    let t4054 = F::new(1.0)/F::sqrt(t344);
    let t4060 = t353 * t964 * t313;
    let t4061 = F::cast_from(0.27385555555555555555e0_f64) * t4060;
    let t4065 = t24 * t1311;
    let t4079 = t1232 * t1232;
    let t4080 = F::new(1.0) / t4079;
    let t4081 = t346 * t4080;
    (t4030, t4031, t4037, t4041, t4049, t4054, t4060, t4061, t4065, t4079, t4080, t4081)
}
