//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 408/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk408<F: Float>(t2484: F, t818: F, t815: F, t823: F, t161: F, t2491: F, t2490: F, t774: F, t755: F, t2526: F, t159: F, t64: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2608 = t2484 * t818;
    let t2610 = t815 * t823;
    let t2612 = t161 * t2491;
    let t2613 = t2490 * t2612;
    let t2615 = t823 * t774;
    let t2616 = t755 * t2615;
    let t2618 = t161 * t2526;
    let t2619 = t755 * t2618;
    let t2621 = t159 * t64;
    (t2608, t2610, t2612, t2613, t2615, t2616, t2618, t2619, t2621)
}
