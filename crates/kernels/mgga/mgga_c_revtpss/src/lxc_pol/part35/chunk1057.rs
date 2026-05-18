//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1057/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1057<F: Float>(t2062: F, t867: F, t786: F, t2470: F, t7406: F, t7064: F, t136: F, t2066: F, t2457: F, t25299: F, t25305: F, t7058: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26496 = t2062 * t867;
    let t26497 = t786 * t26496;
    let t26506 = t7406 * t2470;
    let t26508 = F::new(0.17135234354032049604e-1) * t7064 * t26506;
    let t26518 = t2066 * t136;
    let t26519 = t26518 * t2457;
    let t26521 = F::new(0.17135234354032049604e-2) * t25299 * t26519;
    let t26534 = F::new(0.22849835011101738147e-2) * t25305 * t26519;
    let t26536 = F::new(0.96373646535613327357e-2) * t7058 * t26506;
    (t26496, t26497, t26506, t26508, t26518, t26519, t26521, t26534, t26536)
}
