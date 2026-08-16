//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 531/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk531(t161: f64, t2491: f64, t2490: f64, t774: f64, t823: f64, t755: f64, t2526: f64, t159: f64, t64: f64, t158: f64, t157: f64, t2586: f64, t2589: f64, t2591: f64, t2595: f64, t2598: f64, t2601: f64, t2603: f64, t2606: f64, t2608: f64, t2610: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2612 = t161 * t2491;
    let t2613 = t2490 * t2612;
    let t2615 = t823 * t774;
    let t2616 = t755 * t2615;
    let t2618 = t161 * t2526;
    let t2619 = t755 * t2618;
    let t2621 = t159 * t64;
    let t2622 = 1.0_f64 / t2621;
    let t2623 = t158 * t2622;
    let t2624 = t157 * t2623;
    let t2626 = t2586 / 8.0_f64 - t2589 / 4.0_f64 - t2591 / 2.0_f64 + t2595 / 4.0_f64 + t2598 / 2.0_f64 - t2601 / 8.0_f64 + 3.0_f64 / 4.0_f64 * t2603 - t2606 / 64.0_f64 + t2608 / 32.0_f64 + t2610 / 8.0_f64 - t2613 / 32.0_f64 - t2616 / 8.0_f64 + t2619 / 64.0_f64 - 5.0_f64 / 16.0_f64 * t2624;
    (t2612, t2613, t2615, t2616, t2618, t2619, t2621, t2622, t2623, t2624, t2626)
}
