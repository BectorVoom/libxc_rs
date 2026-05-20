//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1205/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1205<F: Float>(t10871: F, t836: F, t18426: F, t4364: F, t221: F, t2485: F, t5978: F, t2484: F, t10552: F, t10554: F, t14317: F, t18261: F, t18262: F, t18265: F, t18267: F, t18300: F, t18301: F, t18308: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F) {
    let t18525 = t10871 * t836;
    let t18527 = t4364 * t18426 * t18525;
    let t18531 = t2485 * t221 * t5978;
    let t18532 = t2484 * t18531;
    let t18534 = t18261 + t18262 + t18265 + t18267 - t9278 + t9308 + t9316 + t9329 + t9333 + t18300 + t18301 + t14317 + t18308 - t10552 + t10554;
    (t18525, t18527, t18532, t18534)
}
