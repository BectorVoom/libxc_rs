//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2742/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2742<F: Float>(t52: F, t12606: F, t12652: F, t12874: F, t16558: F, t16563: F, t16568: F, t2244: F, t2250: F, t2440: F, t40647: F, t4087: F, t5392: F, t5398: F, t55677: F, t55723: F, t607: F, t76: F, t9438: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t57873 = piecewise3::<F>(t150, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t40647 * t5392 * t2244 + F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t12874 * t12652 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t16563 * t2250 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2440 * t55723 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4087 * t12606 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9438 * t5398 * t2244 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2440 * t16558 * t607 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t16568 * t2250 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t76 * t55677);
    t57873
}
