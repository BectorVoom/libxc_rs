//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1706/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1706<F: Float>(t6258: F, t6299: F, t1651: F, t23820: F, t1042: F, t1045: F, t1063: F, t11859: F, t15618: F, t16067: F, t16199: F, t16208: F, t19501: F, t23908: F, t23921: F, t24009: F, t3092: F, t3115: F, t3117: F, t3155: F, t43174: F, t4837: F, t4872: F, t4892: F, t53800: F, t5819: F, t5825: F, t6244: F, t6266: F, t66721: F, t66763: F, t78496: F, t79638: F, t88732: F, t88804: F) -> (F, F, F) {
    let t89240 = t6258 * t6299;
    let t89245 = t1651 * t23820;
    let t89250 = F::cast_from(0.17149607247227894789e-2_f64) * t4837 * t1042 * t4872 * t5825 * t6244 - F::cast_from(0.85748036236139473944e-2_f64) * t1063 * t1042 * t16199 * t88732 + F::cast_from(0.38110238327173099532e-2_f64) * t1063 * t1042 * t16208 * t88732 + F::cast_from(7.0_f64) / F::cast_from(486.0_f64) * t79638 - F::cast_from(0.51448821741683684368e-2_f64) * t53800 * t24009 - F::cast_from(0.25724410870841842184e-2_f64) * t11859 * t3117 * t88804 * t3155 - F::cast_from(0.34299214494455789577e-2_f64) * t15618 * t23921 - F::cast_from(0.34299214494455789578e-2_f64) * t4892 * t3092 * t19501 * t43174 * t5819 + F::cast_from(0.57165357490759649296e-3_f64) * t16067 * t3092 * t78496 * t6266 + F::cast_from(0.17149607247227894789e-2_f64) * t15618 * t23908 - t66721 / F::cast_from(216.0_f64) - F::cast_from(0.38110238327173099531e-3_f64) * t66763 - F::cast_from(0.12862205435420921092e-2_f64) * t3115 * t3117 * t89240 * t1045 - F::cast_from(0.85748036236139473944e-3_f64) * t3115 * t3117 * t89245 * t1045;
    (t89240, t89245, t89250)
}
