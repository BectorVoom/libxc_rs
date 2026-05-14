//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1073/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1073<F: Float>(t126323: F, t126327: F, t121809: F, t27186: F, t121901: F, t120045: F, t120048: F, t120057: F, t121846: F, t121980: F, t121990: F, t126319: F, t126325: F, t126340: F, t27312: F, t127767: F, t7060: F, t786: F) -> (F, F) {
    let t127809 = 0.150583822711895824e-3 * t126323;
    let t127811 = 0.1054086758983270768e-1 * t126327;
    let t127814 = t121809 * t27186;
    let t127816 = t121901 * t27186;
    let t127821 = -0.225875734067843736e-2 * t126319 + t127809 - 0.26773803678175077509e-3 * t126325 + t127811 - t120045 - 0.69416347856895220196e-2 * t120048 - t121980 + 0.56468933516960933999e-3 * t126340 + 0.28559868832551176308e-1 * t127814 - 0.50779446784275991476e-1 * t127816 + 0.3427184259906141157e1 * t120057 * t121846 * t27312 + t121990;
    let t127827 = t786 * t127767 * t7060;
    (t127821, t127827)
}
