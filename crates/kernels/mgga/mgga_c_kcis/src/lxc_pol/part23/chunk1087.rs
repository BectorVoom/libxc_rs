//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1087/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1087<F: Float>(t17417: F, t28624: F, t27514: F, t5932: F, t1532: F, t572: F, t15911: F, t97782: F, t97785: F, t97787: F, t97789: F, t97791: F, t97794: F, t97796: F, t97798: F, t97802: F, t97805: F, t97807: F, t97809: F, t97811: F, t97813: F, t97815: F) -> (F, F, F, F) {
    let t97817 = t28624 * t17417;
    let t97819 = t27514 * t5932;
    let t97821 = t1532 * t572;
    let t97822 = t97821 * t15911;
    let t97824 = 0.28777777777777777778e0 * t97782 - 0.33333333333333333334e0 * t97785 + 0.26979166666666666667e-1 * t97787 + 0.20234375e-1 * t97789 + 0.26979166666666666667e-1 * t97791 + 0.25e0 * t97794 + 0.27777777777777777777e-1 * t97796 - 0.41666666666666666666e-1 * t97798 - 0.28777777777777777778e0 * t97802 - 0.5e0 * t97805 + 0.53958333333333333334e-1 * t97807 - 0.25e0 * t97809 + 0.25e0 * t97811 + 0.10791666666666666667e0 * t97813 - 0.9375e-1 * t97815 - 0.809375e-1 * t97817 - 0.125e0 * t97819 + 0.41666666666666666666e-1 * t97822;
    (t97817, t97819, t97822, t97824)
}
