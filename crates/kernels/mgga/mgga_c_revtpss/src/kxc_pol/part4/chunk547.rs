//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 547/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk547<F: Float>(t2629: F, t2630: F, t2392: F, t2400: F, t2402: F, t2416: F, t2498: F, t2518: F, t2522: F, t2525: F, t2527: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t2614: F, t2617: F, t2621: F, t2624: F, t2628: F) -> (F, F) {
    let t2632 = 0.10843581300301739842e-1 * t2629 * t2630;
    let t2633 = -t2498 - t2518 - t2522 - t2525 + t2402 + t2527 + t2610 + t2579 + t2587 + t2614 + t2416 - t2562 + t2400 + t2617 - t2569 + t2621 - t2624 + t2628 + t2632 + t2392;
    (t2632, t2633)
}
