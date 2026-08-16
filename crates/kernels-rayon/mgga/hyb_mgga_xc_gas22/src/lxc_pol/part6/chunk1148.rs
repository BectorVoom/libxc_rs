//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1148/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1148(t1123: f64, t4530: f64, t1129: f64, t4540: f64, t1145: f64, t4544: f64, t1128: f64, t4574: f64, t1150: f64, t11515: f64, t11521: f64, t11525: f64, t2868: f64, t2875: f64, t2881: f64, t2922: f64, t2927: f64, t3720: f64, t3724: f64, t7721: f64, t7739: f64, t7769: f64, t7775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11536 = t4530 * t1123;
    let t11540 = t4530 * t1129;
    let t11544 = t4540 * t1123;
    let t11548 = t4540 * t1129;
    let t11549 = t1145 * t11548;
    let t11552 = t4544 * t1123;
    let t11553 = t1145 * t11552;
    let t11556 = t4544 * t1129;
    let t11557 = t1145 * t11556;
    let t11570 = t4574 * t1128;
    let t11573 = 6.0_f64 * t7739 * t11521 - 12.0_f64 * t7769 * t11525 + 60.0_f64 * t7775 * t1145 * t11536 - 90.0_f64 * t7721 * t1145 * t11540 + 15.0_f64 * t2868 * t1145 * t11544 - 18.0_f64 * t2922 * t11549 - 18.0_f64 * t2922 * t11553 + 21.0_f64 * t2875 * t11557 - 2.0_f64 * t2927 * t11549 - 2.0_f64 * t2927 * t11553 + 3.0_f64 * t2881 * t11557 + 800.0_f64 / 9.0_f64 * t3720 * t11515 + 800.0_f64 / 9.0_f64 * t3724 * t11515 - 2.0_f64 * t11570 * t1150;
    (t11536, t11540, t11544, t11548, t11549, t11552, t11556, t11570, t11573)
}
