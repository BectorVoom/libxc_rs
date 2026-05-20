//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2987/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2987<F: Float>(t11859: F, t11922: F, t24008: F, t23820: F, t73: F, t23934: F, t999: F, t1651: F, t19477: F, t1043: F, t1045: F, t11631: F, t11875: F, t15700: F, t15906: F, t16081: F, t16222: F, t1668: F, t19572: F, t19634: F, t19639: F, t19682: F, t19688: F, t23929: F, t23997: F, t24009: F, t3115: F, t3117: F, t42274: F, t42643: F, t4834: F, t4910: F, t53543: F, t54916: F, t6273: F, t65144: F, t65801: F, t65803: F, t65807: F, t79101: F) -> (F, F, F, F) {
    let t79155 = t11859 * t11922 * t24008;
    let t79159 = t23820 * t73;
    let t79175 = t23934 * t999;
    let t79180 = t1651 * t19477;
    let t79206 = -F::cast_from(0.85748036236139473947e-3_f64) * t79155 - F::cast_from(0.12862205435420921092e-2_f64) * t42643 * t24009 - F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t3117 * t79159 * t4910 + F::cast_from(0.38586616306262763276e-2_f64) * t16081 * t3117 * t65144 * t11631 * t1668 * t1043 - F::cast_from(0.38586616306262763276e-2_f64) * t15906 * t3117 * t65144 * t23929 * t1043 + F::cast_from(0.64311027177104605458e-3_f64) * t11875 * t3117 * t19572 * t79175 - F::cast_from(0.64311027177104605458e-3_f64) * t3115 * t3117 * t79180 * t1045 + F::cast_from(0.68598428988911579157e-2_f64) * t54916 * t6273 - F::cast_from(0.12862205435420921092e-2_f64) * t11859 * t3117 * t23997 * t19634 + F::cast_from(0.64311027177104605458e-3_f64) * t11875 * t3117 * t23997 * t19639 - F::cast_from(0.63517063878621832551e-4_f64) * t42274 - t53543 + F::cast_from(0.71456696863449561621e-3_f64) * t15700 * t16222 * t79101 + F::cast_from(0.47637797908966374413e-3_f64) * t65801 + F::cast_from(0.28582678745379824648e-3_f64) * t65803 + F::cast_from(0.28582678745379824648e-3_f64) * t65807 - F::cast_from(0.85748036236139473944e-3_f64) * t4834 * t19682 + F::cast_from(0.7145669686344956162e-3_f64) * t4834 * t19688;
    (t79159, t79175, t79180, t79206)
}
