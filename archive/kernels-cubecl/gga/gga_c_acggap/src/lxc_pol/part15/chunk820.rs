//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 820/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk820<F: Float>(t495: F, t560: F, t1734: F, t469: F, t157: F, t524: F, t556: F, t1907: F, t615: F, t1745: F, t589: F, t137: F, t1713: F) -> (F, F, F, F, F, F) {
    let t9476 = t495 * t560;
    let t9480 = t469 * t1734;
    let t9508 = t556 * t524 * t157;
    let t9517 = t615 * t1907;
    let t9522 = t589 * t1745;
    let t9529 = t137 * t1713;
    (t9476, t9480, t9508, t9517, t9522, t9529)
}
