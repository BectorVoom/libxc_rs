//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 981/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk981<F: Float>(t22805: F, t4742: F, t10757: F, t8549: F, t1663: F, t10755: F, t16541: F, t6807: F, t16462: F, t1674: F, t1686: F, t22747: F, t22750: F, t22757: F, t22761: F, t22764: F, t22786: F, t22788: F, t22791: F, t22794: F, t22797: F, t22800: F, t22804: F, t2396: F, t4757: F, t8609: F) -> (F, F, F, F) {
    let t22807 = 0.32163648644302209644e2 * t4742 * t22805;
    let t22808 = t8549 * t10757;
    let t22809 = t22808 * t1663;
    let t22811 = 0.51725014705706168417e3 * t10755 * t22809;
    let t22813 = 4.0 * t16541 * t6807;
    let t22814 = -0.58482233974552040708e0 * t1674 * t22747 - 0.58482233974552040708e0 * t22750 * t1686 - 0.11696446794910408142e1 * t16462 * t2396 - 0.1025389702100779493e4 * t1674 * t22757 - 0.17315755899375863299e2 * t1674 * t22761 - 0.34631511798751726598e2 * t1674 * t22764 - 0.58482233974552040708e0 * t4757 * t8609 - t22786 + t22788 + t22791 - t22794 - t22797 - t22800 + t22804 + t22807 + t22811 - t22813;
    (t22807, t22811, t22813, t22814)
}
