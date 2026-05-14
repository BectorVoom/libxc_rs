//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1079/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1079<F: Float>(t1685: F, t22745: F, t1683: F, t9137: F, t12084: F, t1979: F, t22788: F, t22791: F, t22794: F, t22797: F, t22800: F, t22804: F, t22807: F, t22811: F, t22813: F, t5405: F, t5408: F, t9134: F) -> (F,) {
    let t24739 = t22745 * t1685;
    let t24742 = t9137 * t1683;
    let t24745 = -0.11696446794910408142e1 * t12084 * t9134 + 0.58482233974552040708e0 * t5405 * t9137 + 0.58482233974552040708e0 * t1979 * t24739 - t22788 - t22791 + t22794 + t22797 + t22800 - t22804 - t22807 - t22811 + t22813 - 0.11696446794910408142e1 * t5408 * t24742;
    (t24745,)
}
