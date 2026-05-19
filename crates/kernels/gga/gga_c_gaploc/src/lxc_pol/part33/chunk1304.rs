//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1304/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1304<F: Float>(t26763: F, t7030: F, t2389: F, t8229: F, t8331: F, t34239: F, t4391: F, t6964: F, t10525: F, t10526: F, t6689: F, t8411: F) -> (F, F, F, F, F, F) {
    let t34306 = t26763 * t7030;
    let t34307 = F::cast_from(0.29792074959875355558e-1_f64) * t34306;
    let t34308 = t8229 * t2389;
    let t34309 = F::cast_from(0.59584149919750711116e-1_f64) * t34308;
    let t34310 = t8331 * t2389;
    let t34311 = F::cast_from(0.59584149919750711116e-1_f64) * t34310;
    let t34314 = F::cast_from(0.85801175884441024006e1_f64) * t4391 * t6964 * t34239;
    let t34318 = F::cast_from(0.42900587942220512002e1_f64) * t10525 * t10526 * t34239;
    let t34320 = F::cast_from(0.10725146985555128001e1_f64) * t8411 * t6689;
    (t34307, t34309, t34311, t34314, t34318, t34320)
}
