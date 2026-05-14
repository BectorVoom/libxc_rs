//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1084/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1084<F: Float>(t240: F, t8584: F, t1686: F, t18472: F, t1987: F, t22757: F, t22761: F, t22804: F, t22807: F, t22811: F, t22813: F, t22816: F, t22870: F, t22872: F, t22893: F, t22895: F, t22897: F, t22899: F, t22901: F, t2396: F, t5423: F, t6876: F, t7517: F, t8609: F) -> (F,) {
    let t24819 = t240 * t8584;
    let t24826 = t22804 + t22807 + t22811 - t22813 - 0.17315755899375863299e2 * t1987 * t22761 - 0.1025389702100779493e4 * t1987 * t22757 + 0.1038945353962551798e3 * t1987 * t22816 + t22870 + t22872 + t22893 + t22895 + t22897 - t22899 + t22901 - 0.58482233974552040708e0 * t5423 * t8609 - 0.58482233974552040708e0 * t24819 * t1686 - 0.11696446794910408142e1 * t18472 * t2396 - 0.11696446794910408142e1 * t7517 * t6876;
    (t24826,)
}
