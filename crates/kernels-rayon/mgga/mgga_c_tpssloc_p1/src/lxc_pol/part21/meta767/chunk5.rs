//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2650/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2650(t2281: f64, t5489: f64, t5465: f64, t19474: f64, t626: f64, t19483: f64, t19477: f64, t12808: f64, t19473: f64, t19482: f64, t19529: f64, t2331: f64, t2332: f64, t2358: f64, t26129: f64, t29903: f64, t4043: f64, t4067: f64, t45435: f64, t45676: f64, t5464: f64, t5488: f64, t64: f64, t666: f64, t9365: f64) -> f64 {
    let t55531 = t2281 * t5489;
    let t55537 = t2281 * t5465;
    let t55546 = t626 * t19474;
    let t55559 = t626 * t19483;
    let t55561 = t626 * t19477;
    let t55566 = -11.0_f64 / 9.0_f64 * t55531 + 4.0_f64 * t45676 - 3.0_f64 * t29903 * t26129 * t4067 + 22.0_f64 / 9.0_f64 * t55537 + t64 * t2331 * t19529 * t666 / 2.0_f64 + t64 * t19482 * t2358 / 4.0_f64 + 4.0_f64 * t55546 + 3.0_f64 * t64 * t45435 * t5464 * t2332 - 3.0_f64 / 4.0_f64 * t64 * t9365 * t5488 * t2332 + t64 * t4043 * t12808 / 2.0_f64 - 4.0_f64 / 3.0_f64 * t55559 - 8.0_f64 / 3.0_f64 * t55561 - 3.0_f64 / 4.0_f64 * t64 * t19473 * t2358;
    t55566
}
