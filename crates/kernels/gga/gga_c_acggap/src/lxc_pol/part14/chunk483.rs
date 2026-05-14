//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 483/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk483<F: Float>(t2628: F, t271: F, t690: F, t680: F, t686: F, t286: F, t244: F, t712: F, t229: F, t804: F, t243: F, t803: F, t40: F, t218: F, t771: F, t777: F, t779: F) -> (F, F, F, F, F, F) {
    let t2629 = 0.35089341735807877242e1 * t2628;
    let t2631 = t690 * t271;
    let t2632 = t686 * t680 * t2631;
    let t2633 = t286 * t2632;
    let t2634 = 0.51947577317044391277e2 * t2633;
    let t2635 = t712 * t244;
    let t2641 = t229 * t804;
    let t2642 = 12.0 * t2641;
    let t2643 = t243 * t803;
    let t2644 = t40 * t2643;
    let t2654 = t777 * t771 * t779 * t218;
    (t2629, t2634, t2635, t2642, t2644, t2654)
}
