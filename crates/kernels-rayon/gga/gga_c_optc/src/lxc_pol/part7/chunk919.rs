//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 919/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk919(t8553: f64, t8697: f64, t8700: f64, t1102: f64, t3067: f64, t3071: f64, t3075: f64, t8587: f64, t8589: f64, t8591: f64, t8593: f64, t8595: f64, t8598: f64, t8601: f64, t8603: f64, t8606: f64, t8609: f64, t8613: f64, t8618: f64, t8622: f64, t8625: f64) -> (f64, f64, f64, f64, f64) {
    let t8701 = t8697 * t8553 * t8700;
    let t8703 = 0.1025389702100779493e4_f64 * t1102 * t8701;
    let t8705 = 0.17544670192365612213e1_f64 * t3067 * t3071;
    let t8707 = 0.51947267698127589899e2_f64 * t3067 * t3075;
    let t8722 = -0.82785e-1_f64 * t8587 - 0.40256666666666666668e0_f64 * t8589 + 0.30192500000000000001e0_f64 * t8591 + 0.20128333333333333333e0_f64 * t8593 - 0.33114e0_f64 * t8595 + 0.16557e0_f64 * t8598 - 0.49671e0_f64 * t8601 - 0.60385000000000000001e0_f64 * t8603 + 0.12077e1_f64 * t8606 - 0.181155e1_f64 * t8609 - 0.412621875e-1_f64 * t8613 + 0.19419375e1_f64 * t8618 - 0.33547222222222222222e0_f64 * t8622 - 0.301925e0_f64 * t8625;
    (t8701, t8703, t8705, t8707, t8722)
}
