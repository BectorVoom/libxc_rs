//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2232/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2232(t10277: f64, t381: f64, t225: f64, t25608: f64, t23384: f64, t25714: f64, t12648: f64, t14165: f64, t14644: f64, t23327: f64, t23346: f64, t23613: f64, t23686: f64, t25429: f64, t25456: f64, t25470: f64, t25510: f64, t25511: f64, t25517: f64, t3010: f64, t6687: f64, t6786: f64, t6797: f64, t6799: f64, t6800: f64, t7614: f64, t82618: f64, t82629: f64, t82633: f64, t82635: f64) -> f64 {
    let t89071 = t381 * t10277;
    let t89076 = t25608 * t225;
    let t89094 = 0.54831135561607547884e-2_f64 * t23384 * t25714;
    let t89101 = -0.54831135561607547884e-2_f64 * t23327 * t25510 * t25511 * t12648 - 0.21932454224643019154e-1_f64 * t25429 * t25510 * t89071 * t14165 - 0.54831135561607547884e-2_f64 * t23327 * t89076 * t6786 - 0.54831135561607547884e-2_f64 * t23327 * t25470 * t23686 + 0.16449340668482264365e-1_f64 * t6797 * t6799 * t14644 * t6800 + 0.43864908449286038306e-1_f64 * t23346 * t25456 - 0.54831135561607547884e-2_f64 * t82618 - 0.54831135561607547884e-2_f64 * t23327 * t23613 * t25517 - t89094 - 0.82246703342411321825e-2_f64 * t6687 * t3010 * t7614 + 0.14621636149762012769e-1_f64 * t82629 + 0.36554090374405031922e-2_f64 * t82633 - 0.12184696791468343974e-2_f64 * t82635;
    t89101
}
