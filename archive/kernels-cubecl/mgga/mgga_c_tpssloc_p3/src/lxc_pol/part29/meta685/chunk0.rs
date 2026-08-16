//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2338/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2338<F: Float>(t27331: F, t9231: F, t2110: F, t22519: F, t22531: F, t22537: F, t24511: F, t24526: F, t26090: F, t27332: F, t6492: F, t7246: F, t7432: F, t7435: F, t7975: F, t7978: F, t85514: F, t85524: F, t90297: F, t90337: F, t90340: F) -> F {
    let t95981 = t9231 * t27331;
    let t95996 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t22519 * t7978 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t24526 * t26090 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t85514 * t7432 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t85524 * t7432 + t7435 * t24511 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t95981 * t6492 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t27332 * t22531 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t22519 * t7975 + t22537 * t7975 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t90297 + t90337 * t2110 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t90340 * t2110;
    t95996
}
