//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1488/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1488(t19473: f64, t20342: f64, t2331: f64, t4043: f64, t45421: f64, t45435: f64, t45656: f64, t5488: f64, t55531: f64, t55537: f64, t64: f64, t656: f64, t75592: f64, t75601: f64, t75613: f64, t79748: f64, t79755: f64, t79812: f64) -> f64 {
    let t79816 = t45421 + 616.0_f64 / 27.0_f64 * t45656 + 44.0_f64 / 3.0_f64 * t55537 - 22.0_f64 / 3.0_f64 * t55531 + 8.0_f64 * t75592 - 8.0_f64 * t75601 + 4.0_f64 / 3.0_f64 * t75613 + 3.0_f64 * t64 * t45435 * t79748 - 9.0_f64 / 2.0_f64 * t64 * t19473 * t5488 + 3.0_f64 / 4.0_f64 * t64 * t2331 * t79755 + t64 * t4043 * t20342 - t64 * t656 * t79812 / 8.0_f64;
    t79816
}
