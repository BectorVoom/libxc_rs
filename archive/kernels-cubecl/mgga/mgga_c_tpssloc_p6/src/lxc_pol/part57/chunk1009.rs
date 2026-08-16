//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1009/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1009<F: Float>(t5445: F, t8513: F, t8514: F, t31691: F, t5441: F, t115833: F, t126065: F, t115903: F, t126073: F, t5392: F, t5389: F, t115834: F, t115871: F, t115907: F, t121094: F, t121121: F, t121124: F, t126046: F, t126062: F, t126091: F, t31675: F, t31681: F, t33560: F, t33564: F, t33572: F, t7026: F, t8512: F) -> F {
    let t128337 = t8513 * t8514 * t5445;
    let t128345 = t8513 * t31691 * t5441;
    let t128352 = t115833 * t126065;
    let t128355 = t115903 * t126073;
    let t128359 = t8513 * t8514 * t5392;
    let t128363 = t8513 * t8514 * t5389;
    let t128368 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t31675 * t126046 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t31675 * t128337 - F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t126091 * t115834 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t8512 * t126062 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8512 * t128345 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t121094 * t33564 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t33560 * t33572 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t115907 * t128352 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t31681 * t128355 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t7026 * t128359 - F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t115871 * t128363 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t121121 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t121124;
    t128368
}
