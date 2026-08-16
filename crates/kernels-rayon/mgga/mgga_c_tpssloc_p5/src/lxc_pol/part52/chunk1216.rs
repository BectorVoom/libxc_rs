//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1216/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1216(t28: f64, t7540: f64, t1649: f64, t1877: f64, t2522: f64, t30757: f64, t30770: f64, t32886: f64, t6670: f64, t7649: f64, t7656: f64, t8366: f64, t8370: f64) -> (f64, f64) {
    let t33065 = t28 * t7540;
    let t33073 = 3.0_f64 / 2.0_f64 * t2522 * t8366 * t7649 + t1877 * t32886 * t28 / 2.0_f64 - t1877 * t30757 * t7656 / 2.0_f64 + t1877 * t8366 * t1649 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2522 * t8370 * t7649 - t1877 * t6670 * t33065 + t1877 * t30770 * t7656 - t1877 * t8370 * t1649 / 2.0_f64;
    (t33065, t33073)
}
