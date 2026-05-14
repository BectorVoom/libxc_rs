//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 801/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk801<F: Float>(t25412: F, t26481: F, t25411: F, t2466: F, t25387: F, t2062: F, t867: F, t786: F, t2467: F, t25431: F, t2470: F, t7406: F, t7064: F, t136: F, t2066: F, t2457: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26482 = t26481 * t25412;
    let t26483 = t25411 * t26482;
    let t26485 = t26481 * t2466;
    let t26486 = t25387 * t26485;
    let t26496 = t2062 * t867;
    let t26497 = t786 * t26496;
    let t26498 = t26497 * t2467;
    let t26500 = t25431 * t26482;
    let t26506 = t7406 * t2470;
    let t26508 = 0.17135234354032049604e-1 * t7064 * t26506;
    let t26518 = t2066 * t136;
    let t26519 = t26518 * t2457;
    (t26483, t26485, t26486, t26497, t26498, t26500, t26506, t26508, t26519)
}
