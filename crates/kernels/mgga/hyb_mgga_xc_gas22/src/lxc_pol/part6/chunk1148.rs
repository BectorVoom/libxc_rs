//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1148/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1148<F: Float>(t1123: F, t4530: F, t1129: F, t4540: F, t1145: F, t4544: F, t1128: F, t4574: F, t1150: F, t11515: F, t11521: F, t11525: F, t2868: F, t2875: F, t2881: F, t2922: F, t2927: F, t3720: F, t3724: F, t7721: F, t7739: F, t7769: F, t7775: F) -> (F, F, F, F, F, F, F, F, F) {
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
    let t11573 = F::new(6.0) * t7739 * t11521 - F::new(12.0) * t7769 * t11525 + F::new(60.0) * t7775 * t1145 * t11536 - F::new(90.0) * t7721 * t1145 * t11540 + F::new(15.0) * t2868 * t1145 * t11544 - F::new(18.0) * t2922 * t11549 - F::new(18.0) * t2922 * t11553 + F::new(21.0) * t2875 * t11557 - F::new(2.0) * t2927 * t11549 - F::new(2.0) * t2927 * t11553 + F::new(3.0) * t2881 * t11557 + F::new(800.0) / F::new(9.0) * t3720 * t11515 + F::new(800.0) / F::new(9.0) * t3724 * t11515 - F::new(2.0) * t11570 * t1150;
    (t11536, t11540, t11544, t11548, t11549, t11552, t11556, t11570, t11573)
}
