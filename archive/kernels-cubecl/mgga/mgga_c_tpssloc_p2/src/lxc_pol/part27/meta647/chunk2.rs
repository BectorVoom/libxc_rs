//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2232/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2232<F: Float>(t10277: F, t381: F, t225: F, t25608: F, t23384: F, t25714: F, t12648: F, t14165: F, t14644: F, t23327: F, t23346: F, t23613: F, t23686: F, t25429: F, t25456: F, t25470: F, t25510: F, t25511: F, t25517: F, t3010: F, t6687: F, t6786: F, t6797: F, t6799: F, t6800: F, t7614: F, t82618: F, t82629: F, t82633: F, t82635: F) -> F {
    let t89071 = t381 * t10277;
    let t89076 = t25608 * t225;
    let t89094 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25714;
    let t89101 = -F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t25510 * t25511 * t12648 - F::cast_from(0.21932454224643019154e-1_f64) * t25429 * t25510 * t89071 * t14165 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t89076 * t6786 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t25470 * t23686 + F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t6799 * t14644 * t6800 + F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25456 - F::cast_from(0.54831135561607547884e-2_f64) * t82618 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23613 * t25517 - t89094 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t3010 * t7614 + F::cast_from(0.14621636149762012769e-1_f64) * t82629 + F::cast_from(0.36554090374405031922e-2_f64) * t82633 - F::cast_from(0.12184696791468343974e-2_f64) * t82635;
    t89101
}
