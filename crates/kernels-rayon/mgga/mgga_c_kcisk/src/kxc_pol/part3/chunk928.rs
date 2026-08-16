//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 928/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk928(t13665: f64, t13522: f64, t13533: f64, t13536: f64, t13540: f64, t13543: f64, t13549: f64, t13555: f64, t13650: f64, t13653: f64, t13656: f64, t13659: f64, t13662: f64, t13670: f64) -> f64 {
    let t13746 = 0.73586666666666666667e0_f64 * t13665;
    let t13748 = 0.93932222222222222223e0_f64 * t13522;
    let t13749 = -0.60385000000000000001e0_f64 * t13533 + 0.30192500000000000001e0_f64 * t13536 - 0.33547222222222222222e0_f64 * t13540 + 0.12077e1_f64 * t13543 - 0.181155e1_f64 * t13549 - 0.301925e0_f64 * t13555 + 0.33114e0_f64 * t13650 - 0.3883875e1_f64 * t13653 + 0.247573125e0_f64 * t13656 - 0.16557e0_f64 * t13659 + 0.99342e0_f64 * t13662 - t13746 + 0.19419375e1_f64 * t13670 - t13748;
    t13749
}
