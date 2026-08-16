//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3046/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3046(t51548: f64, t786: f64, t10532: f64, t40270: f64, t4496: f64, t136: f64, t137: f64, t14597: f64, t2438: f64, t2723: f64, t49180: f64, t836: f64) -> (f64, f64, f64) {
    let t51549 = t786 * t51548;
    let t51550 = t51549 * t10532;
    let t51553 = t40270 * t4496;
    let t51560 = t49180 * t14597 * t2723 * t136 * t137 * t2438 * t836;
    (t51550, t51553, t51560)
}
