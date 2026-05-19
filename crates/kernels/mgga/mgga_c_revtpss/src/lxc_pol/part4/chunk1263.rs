//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1263/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1263<F: Float>(t11986: F, t1592: F, t247: F, t1063: F, t1062: F, t11940: F, t1651: F, t3059: F, t3116: F, t11672: F, t11675: F, t11712: F, t11774: F, t15684: F, t15689: F, t15693: F, t15697: F, t15700: F, t15703: F, t15707: F, t3101: F, t3106: F, t3130: F, t4788: F, t4831: F, t4834: F) -> (F, F) {
    let t15711 = t247 * t11986 * t1592;
    let t15712 = t1063 * t15711;
    let t15716 = t11940 * t1062;
    let t15717 = t1651 * t3059;
    let t15719 = t247 * t3116 * t15717;
    let t15722 = -F::cast_from(0.15244095330869239812e-2_f64) * t3106 * t4831 - F::cast_from(0.15244095330869239812e-2_f64) * t11672 * t4788 + t15684 + F::cast_from(0.28582678745379824648e-3_f64) * t11675 * t4788 - F::cast_from(0.28582678745379824648e-3_f64) * t15689 * t15693 - F::cast_from(0.28582678745379824648e-3_f64) * t11774 * t15697 - F::cast_from(0.57165357490759649296e-3_f64) * t15700 * t15703 + F::cast_from(0.19055119163586549765e-3_f64) * t11712 - F::cast_from(0.28582678745379824648e-3_f64) * t15707 * t3130 - F::cast_from(0.31758531939310916276e-4_f64) * t15712 - F::cast_from(0.28582678745379824648e-3_f64) * t4834 * t3101 - F::cast_from(0.12862205435420921092e-2_f64) * t15716 * t15719;
    (t15717, t15722)
}
