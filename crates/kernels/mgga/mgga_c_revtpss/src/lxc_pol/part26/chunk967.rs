//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 967/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk967<F: Float>(t225: F, t26473: F, t2470: F, t7406: F, t7064: F, t2061: F, t2722: F, t25416: F, t2723: F, t231: F, t7076: F, t136: F, t2066: F, t2457: F, t25299: F, t25365: F, t7407: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26502 = t26473 * t225;
    let t26506 = t7406 * t2470;
    let t26508 = 0.17135234354032049604e-1 * t7064 * t26506;
    let t26509 = t2061 * t2722;
    let t26511 = t25416 * t26509 * t2723;
    let t26515 = t7076 * t26509 * t231;
    let t26518 = t2066 * t136;
    let t26519 = t26518 * t2457;
    let t26521 = 0.17135234354032049604e-2 * t25299 * t26519;
    let t26522 = t25365 * t7407;
    (t26502, t26506, t26508, t26511, t26515, t26518, t26519, t26521, t26522)
}
