//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1374/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1374<F: Float>(t18435: F, t2477: F, t828: F, t14718: F, t6035: F, t2662: F, t2661: F, t125: F, t6016: F, t2747: F, t2749: F, t18426: F) -> (F, F, F, F, F, F) {
    let t18437 = t2477 * t828 * t18435;
    let t18440 = t14718 * t6035;
    let t18441 = t2662 * t18440;
    let t18442 = t2661 * t18441;
    let t18444 = t125 * t6016;
    let t18446 = t2747 * t18444 * t2749;
    let t18451 = t2747 * t18426 * t2749;
    (t18437, t18440, t18442, t18444, t18446, t18451)
}
