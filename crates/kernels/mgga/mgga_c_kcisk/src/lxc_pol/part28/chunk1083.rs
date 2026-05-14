//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1083/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1083<F: Float>(t1987: F, t22747: F, t22764: F, t22786: F, t22788: F, t22791: F, t22794: F, t22797: F, t22800: F, t22820: F, t22888: F, t22908: F, t22915: F, t240: F, t24734: F, t24745: F, t24788: F, t24800: F, t5423: F, t6857: F, t6881: F, t7517: F, t8592: F, t8613: F) -> (F,) {
    let t24810 = 0.19751789702565206229e-1 * t240 * t22888 + 0.11696446794910408142e1 * t1987 * t22820 + 0.23392893589820816284e1 * t7517 * t6857 - 0.34631511798751726598e2 * t1987 * t22764 - 0.346315117987517266e2 * t7517 * t6881 - 0.35089340384731224426e1 * t1987 * t22908 + 0.23392893589820816284e1 * t1987 * t22915 + t240 * (t24734 + t24745 + t24788 + t24800) + 0.11696446794910408142e1 * t5423 * t8592 - t22786 - 0.58482233974552040708e0 * t1987 * t22747 - 0.17315755899375863299e2 * t5423 * t8613 + t22788 + t22791 - t22794 - t22797 - t22800;
    (t24810,)
}
