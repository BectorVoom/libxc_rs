//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 413/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk413<F: Float>(t2366: F, t680: F, t2372: F, t47: F, t2355: F, t2375: F, t119: F, t198: F, t2302: F, t2306: F, t2313: F, t2336: F, t2344: F, t237: F, t2658: F, t2664: F, t2666: F, t2676: F, t2681: F, t2684: F, t2690: F, t2694: F, t2695: F, t5: F, t56: F, t845: F, t852: F, t858: F, t859: F) -> (F, F, F, F) {
    let t2698 = t2366 * t680;
    let t2701 = t47 * t2372;
    let t2702 = t2355 * t2375;
    let t2705 = -F::cast_from(0.70981924444444444442e-3_f64) * t5 * t119 * t198 - F::cast_from(0.34246666666666666666e-1_f64) * t237 * t2658 * t852 - F::cast_from(2.0_f64) * t2664 * t2666 + F::cast_from(1.0_f64) * t845 * t2676 + F::cast_from(0.32164683177870697974e2_f64) * t2681 * t2684 + t2302 + t2306 + t2313 - t2336 - t2344 - F::cast_from(0.24415406715670879921e-3_f64) * t5 * t119 * t56 - F::cast_from(0.10843580882781524214e-1_f64) * t237 * t2690 * t859 - F::cast_from(0.11696446794910408142e1_f64) * t2694 * t2695 + F::cast_from(0.58482233974552040708e0_f64) * t858 * t2698 + F::cast_from(0.17315755899375863299e2_f64) * t2701 * t2702;
    (t2698, t2701, t2702, t2705)
}
