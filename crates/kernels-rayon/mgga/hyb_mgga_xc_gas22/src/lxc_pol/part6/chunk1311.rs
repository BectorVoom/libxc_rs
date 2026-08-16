//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1311/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1311(t10450: f64, t680: f64, t2018: f64, t3990: f64, t10472: f64, t677: f64, t136: f64, t1815: f64, t3946: f64, t10168: f64, t1240: f64, t1243: f64, t139: f64, t2002: f64, t2024: f64, t2027: f64, t2028: f64, t214: f64, t23809: f64, t24021: f64, t24026: f64, t24426: f64, t24439: f64, t26: f64, t28136: f64, t28150: f64, t28153: f64, t28156: f64, t28628: f64, t2949: f64, t2950: f64, t3287: f64, t675: f64, t684: f64, t687: f64, t8536: f64) -> f64 {
    let t28634 = t10450 * t680;
    let t28636 = t3990 * t2018;
    let t28638 = t677 * t10472;
    let t28642 = t136 * t1815 * t3946;
    let t28645 = -3.0_f64 / 32.0_f64 * t1240 * t8536 + 3.0_f64 / 8.0_f64 * t2949 * t2950 * t3287 + t24021 / 24.0_f64 - t684 * t687 * t28136 * t675 / 32.0_f64 - t684 * t687 * t10168 * t2002 / 64.0_f64 - t2024 * t2027 * t10168 * t2028 / 48.0_f64 - t28150 / 96.0_f64 - t28153 / 96.0_f64 - t28156 / 96.0_f64 + t2024 * t23809 * t1243 / 12.0_f64 - t24026 / 32.0_f64 - 3.0_f64 / 64.0_f64 * t136 * t26 * t139 * t28628 * t214 - t28634 / 32.0_f64 - t28636 / 32.0_f64 - t28638 / 16.0_f64 - t24426 / 32.0_f64 + t28642 / 48.0_f64 - t24439 / 32.0_f64;
    t28645
}
