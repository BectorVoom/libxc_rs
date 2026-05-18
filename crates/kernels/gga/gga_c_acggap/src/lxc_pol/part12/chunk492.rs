//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 492/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk492<F: Float>(t2607: F, t31: F, t4: F, t195: F, t682: F, t656: F, t691: F, t243: F, t657: F, t288: F, t668: F, t912: F) -> (F, F, F, F, F, F) {
    let t2609 = t4 * t2607 * t31;
    let t2610 = F::new(0.34450798614814814813e-2) * t2609;
    let t2611 = t195 * t682;
    let t2612 = t656 * t2611;
    let t2613 = F::new(0.16265371950452609763e-1) * t2612;
    let t2614 = t195 * t691;
    let t2615 = t656 * t2614;
    let t2616 = F::new(0.48159733137676571078e0) * t2615;
    let t2617 = t243 * t4;
    let t2618 = t2617 * t657;
    let t2620 = t668 * t288;
    let t2621 = t656 * t2620;
    let t2622 = F::new(0.21687162600603479684e-1) * t2621;
    let t2623 = t195 * t912;
    (t2610, t2613, t2616, t2618, t2622, t2623)
}
