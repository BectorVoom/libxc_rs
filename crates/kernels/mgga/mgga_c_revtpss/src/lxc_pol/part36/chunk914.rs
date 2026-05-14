//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 914/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk914<F: Float>(t23168: F, t827: F, t828: F, t23172: F, t124: F, t23114: F, t800: F, t23148: F, t1544: F, t5984: F, t10673: F, t10687: F, t10692: F, t10870: F, t10900: F, t14712: F, t14716: F, t14761: F, t14765: F, t18338: F, t18340: F, t2721: F, t2730: F, t799: F) -> (F, F, F, F, F) {
    let t23253 = t827 * t828 * t23168;
    let t23257 = t827 * t828 * t23172;
    let t23262 = t124 * t23114;
    let t23263 = t800 * t23262;
    let t23266 = t124 * t23148;
    let t23267 = t800 * t23266;
    let t23275 = t800 * t5984 * t1544;
    let t23278 = t10673 - 0.12862205435420921092e-2 * t10870 * t23253 + 0.12862205435420921092e-2 * t2721 * t23257 - 0.17006693853500995666e-1 * t14712 + 0.40656002247428262579e-3 * t14716 - t10900 * t23263 / 4.0 - t799 * t23267 / 48.0 - 0.13553694749236397037e-4 * t14761 - t10687 + t10692 - 35.0 / 72.0 * t14765 + 7.0 / 48.0 * t18338 - 7.0 / 16.0 * t18340 + 3.0 / 16.0 * t2730 * t23275;
    (t23253, t23257, t23263, t23267, t23278)
}
