//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2691/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2691<F: Float>(t1307: F, t1315: F, t16101: F, t19631: F, t19781: F, t19793: F, t210: F, t213: F, t214: F, t221: F, t3719: F, t3733: F, t3734: F, t40372: F, t5195: F, t54728: F, t56275: F, t56482: F, t56484: F, t56486: F, t56491: F, t56493: F, t56501: F, t56505: F, t56514: F) -> F {
    let t56525 = -F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t210 * t214 * t56275 + F::cast_from(0.16666666666666666666e-2_f64) * t56482 + F::cast_from(0.38888888888888888887e-1_f64) * t56484 + F::cast_from(0.99999999999999999996e-2_f64) * t3733 * t210 * t214 * t56486 - F::cast_from(0.12962962962962962962e-1_f64) * t56491 - F::cast_from(0.23333333333333333332e-1_f64) * t56493 + F::cast_from(0.49999999999999999998e-2_f64) * t5195 * t221 * t19793 * t3719 + F::cast_from(0.19999999999999999999e-1_f64) * t56501 - F::cast_from(0.99999999999999999996e-2_f64) * t56505 + F::cast_from(0.99999999999999999996e-2_f64) * t5195 * t221 * t213 * t19631 * t1307 - F::cast_from(0.49999999999999999998e-2_f64) * t56514 - F::cast_from(0.19999999999999999999e-1_f64) * t16101 * t221 * t19793 * t3734 + F::cast_from(0.99999999999999999995e-1_f64) * t54728 * t221 * t19781 * t3734 + F::cast_from(0.55555555555555555555e-3_f64) * t40372;
    t56525
}
