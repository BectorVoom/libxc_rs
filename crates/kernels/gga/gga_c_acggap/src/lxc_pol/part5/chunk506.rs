//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 506/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk506<F: Float>(t2617: F, t657: F, t288: F, t668: F, t656: F, t195: F, t912: F, t273: F, t680: F, t791: F, t286: F, t271: F, t690: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2618 = t2617 * t657;
    let t2620 = t668 * t288;
    let t2621 = t656 * t2620;
    let t2622 = F::new(0.21687162600603479684e-1) * t2621;
    let t2623 = t195 * t912;
    let t2624 = t656 * t2623;
    let t2625 = F::new(0.32530743900905219526e-1) * t2624;
    let t2627 = t791 * t680 * t273;
    let t2628 = t286 * t2627;
    let t2629 = F::new(0.35089341735807877242e1) * t2628;
    let t2631 = t690 * t271;
    (t2618, t2620, t2621, t2622, t2623, t2624, t2625, t2627, t2628, t2629, t2631)
}
