//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 864/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk864<F: Float>(t8553: F, t8697: F, t8700: F, t1102: F, t3067: F, t3071: F, t3075: F, t8587: F, t8589: F, t8591: F, t8593: F, t8595: F, t8598: F, t8601: F, t8603: F, t8606: F, t8609: F, t8613: F, t8618: F, t8622: F, t8625: F) -> (F, F, F, F, F) {
    let t8701 = t8697 * t8553 * t8700;
    let t8703 = 0.1025389702100779493e4 * t1102 * t8701;
    let t8705 = 0.17544670192365612213e1 * t3067 * t3071;
    let t8707 = 0.51947267698127589899e2 * t3067 * t3075;
    let t8722 = -0.82785e-1 * t8587 - 0.40256666666666666668e0 * t8589 + 0.30192500000000000001e0 * t8591 + 0.20128333333333333333e0 * t8593 - 0.33114e0 * t8595 + 0.16557e0 * t8598 - 0.49671e0 * t8601 - 0.60385000000000000001e0 * t8603 + 0.12077e1 * t8606 - 0.181155e1 * t8609 - 0.412621875e-1 * t8613 + 0.19419375e1 * t8618 - 0.33547222222222222222e0 * t8622 - 0.301925e0 * t8625;
    (t8701, t8703, t8705, t8707, t8722)
}
