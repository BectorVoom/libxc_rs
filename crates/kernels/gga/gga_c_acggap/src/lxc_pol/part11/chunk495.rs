//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 495/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk495<F: Float>(t2623: F, t656: F, t273: F, t680: F, t791: F, t286: F, t271: F, t690: F, t686: F, t244: F, t712: F, t229: F, t804: F) -> (F, F, F, F, F) {
    let t2624 = t656 * t2623;
    let t2625 = F::new(0.32530743900905219526e-1) * t2624;
    let t2627 = t791 * t680 * t273;
    let t2628 = t286 * t2627;
    let t2629 = F::new(0.35089341735807877242e1) * t2628;
    let t2631 = t690 * t271;
    let t2632 = t686 * t680 * t2631;
    let t2633 = t286 * t2632;
    let t2634 = F::new(0.51947577317044391277e2) * t2633;
    let t2635 = t712 * t244;
    let t2641 = t229 * t804;
    (t2625, t2629, t2634, t2635, t2641)
}
