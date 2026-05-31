//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 507/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk507<F: Float>(t4029: F, t346: F, t1253: F, t1254: F, t344: F, t347: F, t1237: F, t4007: F, t4011: F, t4015: F, t4018: F, t4021: F) -> (F, F, F, F, F, F, F, F) {
    let t4030 = F::cast_from(1.0_f64) / t4029;
    let t4031 = t346 * t4030;
    let t4032 = t1253 * t1253;
    let t4033 = t4032 * t1254;
    let t4037 = F::cast_from(1.0_f64) / t347 / t344;
    let t4038 = t1237 * t1237;
    let t4039 = t4037 * t4038;
    let t4041 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4007;
    let t4046 = t4041 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4011 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4015 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4018 - t4021 / F::cast_from(3.0_f64);
    (t4030, t4031, t4032, t4033, t4037, t4038, t4039, t4046)
}
