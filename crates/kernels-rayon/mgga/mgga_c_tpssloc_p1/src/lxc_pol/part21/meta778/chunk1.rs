//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2691/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2691(t1307: f64, t1315: f64, t16101: f64, t19631: f64, t19781: f64, t19793: f64, t210: f64, t213: f64, t214: f64, t221: f64, t3719: f64, t3733: f64, t3734: f64, t40372: f64, t5195: f64, t54728: f64, t56275: f64, t56482: f64, t56484: f64, t56486: f64, t56491: f64, t56493: f64, t56501: f64, t56505: f64, t56514: f64) -> f64 {
    let t56525 = -0.16666666666666666666e-2_f64 * t1315 * t210 * t214 * t56275 + 0.16666666666666666666e-2_f64 * t56482 + 0.38888888888888888887e-1_f64 * t56484 + 0.99999999999999999996e-2_f64 * t3733 * t210 * t214 * t56486 - 0.12962962962962962962e-1_f64 * t56491 - 0.23333333333333333332e-1_f64 * t56493 + 0.49999999999999999998e-2_f64 * t5195 * t221 * t19793 * t3719 + 0.19999999999999999999e-1_f64 * t56501 - 0.99999999999999999996e-2_f64 * t56505 + 0.99999999999999999996e-2_f64 * t5195 * t221 * t213 * t19631 * t1307 - 0.49999999999999999998e-2_f64 * t56514 - 0.19999999999999999999e-1_f64 * t16101 * t221 * t19793 * t3734 + 0.99999999999999999995e-1_f64 * t54728 * t221 * t19781 * t3734 + 0.55555555555555555555e-3_f64 * t40372;
    t56525
}
