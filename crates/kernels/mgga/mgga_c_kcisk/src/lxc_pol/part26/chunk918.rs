//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 918/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk918<F: Float>(t1398: F, t25450: F, t13989: F, t14027: F, t14029: F, t14093: F, t14100: F, t158: F, t165: F, t173: F, t25342: F, t25406: F, t25413: F, t25416: F, t25419: F, t25422: F, t25425: F, t25427: F, t25429: F, t25433: F, t25438: F, t25442: F, t25447: F, t3819: F, t3891: F, t5823: F) -> (F,) {
    let t25451 = t1398 * t25450;
    let t25454 = -0.10929333333333333333e-1 * t13989 + 0.35222222222222222222e-2 * t14027 + 0.39210208333333333333e-4 * t14029 - 0.23911438650126355246e-1 * t3819 * t25406 + 0.15538616723388920628e-3 * t3891 * t25406 + 0.71734315950379065738e-1 * t14093 * t25342 - 0.95645754600505420984e-1 * t14100 * t25413 - 0.3513e-2 * t158 * t25416 + 0.7925e-3 * t165 * t25419 + 0.50413125e-5 * t173 * t25422 - 0.15613333333333333333e-2 * t25425 - 0.13208333333333333333e-2 * t25427 + 0.88055555555555555555e-3 * t25429 - 0.1585e-2 * t165 * t25433 - 0.52833333333333333333e-3 * t165 * t25438 + 0.30247875e-4 * t173 * t25442 + 0.403305e-4 * t173 * t25447 - 0.403305e-4 * t5823 * t25451;
    (t25454,)
}
