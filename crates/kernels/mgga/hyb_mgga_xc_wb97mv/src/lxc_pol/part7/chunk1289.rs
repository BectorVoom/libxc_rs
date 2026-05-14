//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1289/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1289<F: Float>(t132: F, t26273: F, t11374: F, t2594: F, t4372: F, t7258: F, t1003: F, t11554: F, t11560: F, t11563: F, t11568: F, t23116: F, t2572: F, t2590: F, t2598: F, t2605: F, t3608: F, t3622: F, t4359: F, t4393: F, t7231: F, t9261: F, t9262: F, t9265: F, t9297: F, t996: F, zeta_threshold: F) -> (F, F) {
    let t133 = t132 <= zeta_threshold;
    let t31568 = piecewise3(t133, 0.0, -t26273);
    let t31588 = t2594 * t11374;
    let t31592 = t7258 * t4372;
    let t31615 = 0.10389515463408878255e3 * t1003 * t11563 * t7231 + 0.23392894490538584828e1 * t2605 * t11560 - 0.34631718211362927518e2 * t2605 * t11568 - 0.34631718211362927518e2 * t1003 * t31588 * t3622 + 0.10389515463408878255e3 * t1003 * t31592 * t2598 - 0.70178683471615754484e1 * t3608 * t9265 - 0.34631718211362927517e2 * t3608 * t9297 + 0.46785788981077169656e1 * t2605 * t11554 + 0.23392894490538584828e1 * t1003 * t2572 * t11374 * t996 + 0.12304822629859687989e5 * t1003 * t23116 * t4359 * t9261 - 0.35089341735807877242e1 * t1003 * t4393 * t2590 - 0.20508037716432813315e4 * t3608 * t9262;
    (t31568, t31615)
}
