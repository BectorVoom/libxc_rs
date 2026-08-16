//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 924/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk924(t13665: f64, t347: f64, t355: f64, t13633: f64, t13522: f64, t13533: f64, t13536: f64, t13540: f64, t13543: f64, t13549: f64, t13555: f64, t13650: f64, t13653: f64, t13656: f64, t13659: f64, t13662: f64) -> (f64, f64) {
    let t13666 = 0.73028148148148148147e0_f64 * t13665;
    let t13669 = 1.0_f64 / t347 / t355 / 8.0_f64;
    let t13670 = t13669 * t13633;
    let t13672 = 0.93011851851851851854e0_f64 * t13522;
    let t13673 = -0.59793333333333333333e0_f64 * t13533 + 0.29896666666666666667e0_f64 * t13536 - 0.33218518518518518518e0_f64 * t13540 + 0.11958666666666666667e1_f64 * t13543 - 0.17938e1_f64 * t13549 - 0.29896666666666666667e0_f64 * t13555 + 0.32862666666666666666e0_f64 * t13650 - 0.28483875e1_f64 * t13653 + 0.46074375e0_f64 * t13656 - 0.16431333333333333333e0_f64 * t13659 + 0.98587999999999999998e0_f64 * t13662 - t13666 + 0.142419375e1_f64 * t13670 - t13672;
    (t13670, t13673)
}
