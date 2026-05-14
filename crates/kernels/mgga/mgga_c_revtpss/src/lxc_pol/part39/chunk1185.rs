//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1185/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1185<F: Float>(t372: F, t4801: F, t4181: F, t4786: F, t1062: F, t4857: F, t11986: F, t1592: F, t247: F, t1063: F, t11940: F, t1651: F, t3059: F, t3116: F, t11672: F, t11675: F, t11712: F, t11774: F, t15684: F, t15689: F, t15693: F, t15697: F, t15700: F, t3101: F, t3106: F, t3130: F, t4788: F, t4831: F, t4834: F) -> (F, F, F) {
    let t15701 = t372 * t4801;
    let t15702 = t4181 * t4786;
    let t15703 = t15701 * t15702;
    let t15707 = t4857 * t1062;
    let t15711 = t247 * t11986 * t1592;
    let t15712 = t1063 * t15711;
    let t15716 = t11940 * t1062;
    let t15717 = t1651 * t3059;
    let t15719 = t247 * t3116 * t15717;
    let t15722 = -0.15244095330869239812e-2 * t3106 * t4831 - 0.15244095330869239812e-2 * t11672 * t4788 + t15684 + 0.28582678745379824648e-3 * t11675 * t4788 - 0.28582678745379824648e-3 * t15689 * t15693 - 0.28582678745379824648e-3 * t11774 * t15697 - 0.57165357490759649296e-3 * t15700 * t15703 + 0.19055119163586549765e-3 * t11712 - 0.28582678745379824648e-3 * t15707 * t3130 - 0.31758531939310916276e-4 * t15712 - 0.28582678745379824648e-3 * t4834 * t3101 - 0.12862205435420921092e-2 * t15716 * t15719;
    (t15702, t15717, t15722)
}
