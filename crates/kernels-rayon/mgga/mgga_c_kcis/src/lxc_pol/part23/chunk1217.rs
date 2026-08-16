//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1217/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1217(t27514: f64, t5932: f64, t1532: f64, t572: f64, t15911: f64, t97782: f64, t97785: f64, t97787: f64, t97789: f64, t97791: f64, t97794: f64, t97796: f64, t97798: f64, t97802: f64, t97805: f64, t97807: f64, t97809: f64, t97811: f64, t97813: f64, t97815: f64, t97817: f64) -> (f64, f64, f64) {
    let t97819 = t27514 * t5932;
    let t97821 = t1532 * t572;
    let t97822 = t97821 * t15911;
    let t97824 = 0.28777777777777777778e0_f64 * t97782 - 0.33333333333333333334e0_f64 * t97785 + 0.26979166666666666667e-1_f64 * t97787 + 0.20234375e-1_f64 * t97789 + 0.26979166666666666667e-1_f64 * t97791 + 0.25e0_f64 * t97794 + 0.27777777777777777777e-1_f64 * t97796 - 0.41666666666666666666e-1_f64 * t97798 - 0.28777777777777777778e0_f64 * t97802 - 0.5e0_f64 * t97805 + 0.53958333333333333334e-1_f64 * t97807 - 0.25e0_f64 * t97809 + 0.25e0_f64 * t97811 + 0.10791666666666666667e0_f64 * t97813 - 0.9375e-1_f64 * t97815 - 0.809375e-1_f64 * t97817 - 0.125e0_f64 * t97819 + 0.41666666666666666666e-1_f64 * t97822;
    (t97819, t97822, t97824)
}
