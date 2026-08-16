//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 425/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk425(t2354: f64, t47: f64, t2355: f64, t680: f64, t2366: f64, t2372: f64, t2375: f64, t119: f64, t198: f64, t2302: f64, t2306: f64, t2313: f64, t2336: f64, t2344: f64, t237: f64, t2658: f64, t2664: f64, t2666: f64, t2676: f64, t2681: f64, t2684: f64, t2690: f64, t5: f64, t56: f64, t845: f64, t852: f64, t858: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2694 = t47 * t2354;
    let t2695 = t2355 * t680;
    let t2698 = t2366 * t680;
    let t2701 = t47 * t2372;
    let t2702 = t2355 * t2375;
    let t2705 = -0.70981924444444444442e-3_f64 * t5 * t119 * t198 - 0.34246666666666666666e-1_f64 * t237 * t2658 * t852 - 2.0_f64 * t2664 * t2666 + 1.0_f64 * t845 * t2676 + 0.32164683177870697974e2_f64 * t2681 * t2684 + t2302 + t2306 + t2313 - t2336 - t2344 - 0.24415406715670879921e-3_f64 * t5 * t119 * t56 - 0.10843580882781524214e-1_f64 * t237 * t2690 * t859 - 0.11696446794910408142e1_f64 * t2694 * t2695 + 0.58482233974552040708e0_f64 * t858 * t2698 + 0.17315755899375863299e2_f64 * t2701 * t2702;
    (t2694, t2695, t2698, t2701, t2702, t2705)
}
