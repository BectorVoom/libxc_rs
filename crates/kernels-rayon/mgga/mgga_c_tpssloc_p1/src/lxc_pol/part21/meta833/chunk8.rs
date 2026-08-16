//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2949/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2949(t13822: f64, t17752: f64, t973: f64, t17753: f64, t17758: f64, t17778: f64, t2960: f64, t2986: f64, t3008: f64, t343: f64, t4510: f64, t4518: f64, t4546: f64, t5842: f64, t59755: f64, t59763: f64, t61391: f64, t61394: f64, t61397: f64, t61405: f64, t61408: f64) -> f64 {
    let t61422 = t973 * t13822 * t17752;
    let t61424 = -0.74074074074074074073e-3_f64 * t61391 - 0.14814814814814814814e-2_f64 * t61394 + 0.37037037037037037036e-3_f64 * t61397 + 0.16666666666666666666e-2_f64 * t2986 * t4518 * t59763 + 0.13333333333333333332e-1_f64 * t2986 * t4510 * t59755 - 0.49382716049382716048e-3_f64 * t61405 + 0.12345679012345679012e-3_f64 * t61408 + 0.88888888888888888887e-2_f64 * t2960 * t17753 + 0.44444444444444444444e-2_f64 * t2960 * t17758 - 0.83333333333333333332e-3_f64 * t973 * t4546 * t5842 * t3008 * t343 + 0.44444444444444444444e-2_f64 * t2960 * t17778 - 0.11111111111111111111e-2_f64 * t61422;
    t61424
}
