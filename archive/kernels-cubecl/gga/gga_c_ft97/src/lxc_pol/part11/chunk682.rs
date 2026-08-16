//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 682/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk682<F: Float>(t184: F, t9470: F, t1580: F, t185: F, t21: F, t2236: F, t2240: F, t2301: F, t2306: F, t2309: F, t363: F, t5: F, t620: F, t623: F, t650: F, t7745: F, t8614: F, t8724: F, t8732: F, t8739: F, t8744: F, t8751: F, t8754: F) -> (F, F) {
    let t9471 = t9470 * t184;
    let t9478 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t8614 * t650 + t623 * t8724 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t5 * t2236 * t363 + t623 * t8732 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t5 * t620 * t1580 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t623 * t8739 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t2240 * t2306 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t623 * t8744 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t2240 * t2301 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2240 * t2309 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t623 * t8751 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t623 * t8754 + t5 * t9471 * t21 / F::cast_from(4.0_f64) + t5 * t185 * t7745 / F::cast_from(4.0_f64);
    (t9471, t9478)
}
