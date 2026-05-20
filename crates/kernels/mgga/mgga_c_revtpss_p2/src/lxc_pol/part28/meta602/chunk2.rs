//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2081/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2081<F: Float>(t13429: F, t13521: F, t13532: F, t13540: F, t1519: F, t2007: F, t2320: F, t2328: F, t2331: F, t25805: F, t27830: F, t28030: F, t4297: F, t508: F, t649: F, t671: F, t6985: F, t7883: F, t92737: F, t97593: F, t97604: F, t97606: F, t97608: F, t97610: F, t97617: F, t97622: F, t97629: F, t97632: F) -> F {
    let t97635 = -F::new(2.0) * t13429 * t2007 - F::new(2.0) * t13521 * t6985 - F::new(4.0) * t13532 * t6985 - F::new(4.0) * t13540 * t6985 - F::new(2.0) * t1519 * t92737 - F::new(4.0) * t1519 * t97632 - t2320 * t7883 - F::new(2.0) * t2328 * t7883 - F::new(4.0) * t2331 * t28030 - F::new(4.0) * t25805 * t4297 - F::new(2.0) * t27830 * t649 - F::new(2.0) * t508 * t97593 - F::new(4.0) * t671 * t97622 - t97604 - t97606 - t97608 - t97610 - t97617 - t97629;
    t97635
}
