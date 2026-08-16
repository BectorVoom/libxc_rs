//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1217/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1217<F: Float>(t27514: F, t5932: F, t1532: F, t572: F, t15911: F, t97782: F, t97785: F, t97787: F, t97789: F, t97791: F, t97794: F, t97796: F, t97798: F, t97802: F, t97805: F, t97807: F, t97809: F, t97811: F, t97813: F, t97815: F, t97817: F) -> (F, F, F) {
    let t97819 = t27514 * t5932;
    let t97821 = t1532 * t572;
    let t97822 = t97821 * t15911;
    let t97824 = F::cast_from(0.28777777777777777778e0_f64) * t97782 - F::cast_from(0.33333333333333333334e0_f64) * t97785 + F::cast_from(0.26979166666666666667e-1_f64) * t97787 + F::cast_from(0.20234375e-1_f64) * t97789 + F::cast_from(0.26979166666666666667e-1_f64) * t97791 + F::cast_from(0.25e0_f64) * t97794 + F::cast_from(0.27777777777777777777e-1_f64) * t97796 - F::cast_from(0.41666666666666666666e-1_f64) * t97798 - F::cast_from(0.28777777777777777778e0_f64) * t97802 - F::cast_from(0.5e0_f64) * t97805 + F::cast_from(0.53958333333333333334e-1_f64) * t97807 - F::cast_from(0.25e0_f64) * t97809 + F::cast_from(0.25e0_f64) * t97811 + F::cast_from(0.10791666666666666667e0_f64) * t97813 - F::cast_from(0.9375e-1_f64) * t97815 - F::cast_from(0.809375e-1_f64) * t97817 - F::cast_from(0.125e0_f64) * t97819 + F::cast_from(0.41666666666666666666e-1_f64) * t97822;
    (t97819, t97822, t97824)
}
