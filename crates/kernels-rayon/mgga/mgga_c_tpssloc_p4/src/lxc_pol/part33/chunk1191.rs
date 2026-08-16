//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1191/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1191(t28593: f64, t383: f64, t1058: f64, t1920: f64, t23619: f64, t25465: f64, t25508: f64, t28597: f64, t28602: f64, t28605: f64, t28610: f64, t28614: f64, t28618: f64, t28622: f64, t28626: f64, t28631: f64, t3200: f64, t353: f64, t4669: f64, t6687: f64, t6797: f64, t7620: f64) -> (f64, f64) {
    let t28634 = t383 * t28593;
    let t28636 = -t3200 * t28597 + 2.0_f64 * t4669 * t7620 + 2.0_f64 * t1058 * t28602 - 0.16449340668482264365e-1_f64 * t6797 * t28605 - t23619 - 0.54831135561607547884e-2_f64 * t25465 + 0.54831135561607547884e-2_f64 * t6687 * t28610 + 0.27415567780803773942e-2_f64 * t6687 * t28614 - 0.54831135561607547884e-2_f64 * t6687 * t28618 + 0.82246703342411321825e-2_f64 * t6797 * t28622 + 0.16449340668482264365e-1_f64 * t6797 * t28626 + 0.54831135561607547884e-2_f64 * t25508 + 0.82246703342411321825e-2_f64 * t1920 * t28631 + t353 * t28634;
    (t28634, t28636)
}
