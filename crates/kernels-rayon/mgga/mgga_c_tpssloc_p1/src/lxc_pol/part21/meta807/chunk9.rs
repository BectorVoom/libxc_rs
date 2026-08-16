//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2819/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2819(t16625: f64, t2379: f64, t2522: f64, t4314: f64, t47645: f64, t5502: f64, t5544: f64, t59014: f64, t59015: f64, t59016: f64, t59018: f64, t59019: f64, t59020: f64, t59023: f64, t59025: f64, t59027: f64, t9470: f64) -> f64 {
    let t59602 = -6.0_f64 * t16625 * t2379 * t4314 - 3.0_f64 * t2522 * t5544 * t9470 + 12.0_f64 * t47645 * t5502 + t59014 + t59015 + t59016 + t59018 + t59019 + t59020 + t59023 + t59025 + t59027;
    t59602
}
