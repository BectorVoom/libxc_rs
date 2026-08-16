//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1156/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1156(t11506: f64, t39324: f64, t12574: f64, t481: f64, t10997: f64, t3262: f64, t40677: f64, t3579: f64, t39332: f64, t1065: f64, t2892: f64, t3270: f64) -> (f64, f64, f64, f64, f64) {
    let t42818 = 3.0_f64 / 2.0_f64 * t11506 * t39324;
    let t42819 = t12574 * t481;
    let t42822 = 135.0_f64 / 64.0_f64 * t3262 * t10997 * t42819;
    let t42824 = 3.0_f64 / 2.0_f64 * t11506 * t40677;
    let t42826 = 5.0_f64 / 8.0_f64 * t3579 * t39332;
    let t42829 = t1065 * t2892;
    let t42830 = t3270 * t42829;
    (t42818, t42822, t42824, t42826, t42830)
}
