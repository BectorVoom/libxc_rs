//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 531/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk531<F: Float>(t161: F, t2491: F, t2490: F, t774: F, t823: F, t755: F, t2526: F, t159: F, t64: F, t158: F, t157: F, t2586: F, t2589: F, t2591: F, t2595: F, t2598: F, t2601: F, t2603: F, t2606: F, t2608: F, t2610: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2612 = t161 * t2491;
    let t2613 = t2490 * t2612;
    let t2615 = t823 * t774;
    let t2616 = t755 * t2615;
    let t2618 = t161 * t2526;
    let t2619 = t755 * t2618;
    let t2621 = t159 * t64;
    let t2622 = F::new(1.0) / t2621;
    let t2623 = t158 * t2622;
    let t2624 = t157 * t2623;
    let t2626 = t2586 / F::new(8.0) - t2589 / F::new(4.0) - t2591 / F::new(2.0) + t2595 / F::new(4.0) + t2598 / F::new(2.0) - t2601 / F::new(8.0) + F::new(3.0) / F::new(4.0) * t2603 - t2606 / F::new(64.0) + t2608 / F::new(32.0) + t2610 / F::new(8.0) - t2613 / F::new(32.0) - t2616 / F::new(8.0) + t2619 / F::new(64.0) - F::new(5.0) / F::new(16.0) * t2624;
    (t2612, t2613, t2615, t2616, t2618, t2619, t2621, t2622, t2623, t2624, t2626)
}
