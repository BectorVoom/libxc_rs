//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1706/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1706(t6258: f64, t6299: f64, t1651: f64, t23820: f64, t1042: f64, t1045: f64, t1063: f64, t11859: f64, t15618: f64, t16067: f64, t16199: f64, t16208: f64, t19501: f64, t23908: f64, t23921: f64, t24009: f64, t3092: f64, t3115: f64, t3117: f64, t3155: f64, t43174: f64, t4837: f64, t4872: f64, t4892: f64, t53800: f64, t5819: f64, t5825: f64, t6244: f64, t6266: f64, t66721: f64, t66763: f64, t78496: f64, t79638: f64, t88732: f64, t88804: f64) -> (f64, f64, f64) {
    let t89240 = t6258 * t6299;
    let t89245 = t1651 * t23820;
    let t89250 = 0.17149607247227894789e-2_f64 * t4837 * t1042 * t4872 * t5825 * t6244 - 0.85748036236139473944e-2_f64 * t1063 * t1042 * t16199 * t88732 + 0.38110238327173099532e-2_f64 * t1063 * t1042 * t16208 * t88732 + 7.0_f64 / 486.0_f64 * t79638 - 0.51448821741683684368e-2_f64 * t53800 * t24009 - 0.25724410870841842184e-2_f64 * t11859 * t3117 * t88804 * t3155 - 0.34299214494455789577e-2_f64 * t15618 * t23921 - 0.34299214494455789578e-2_f64 * t4892 * t3092 * t19501 * t43174 * t5819 + 0.57165357490759649296e-3_f64 * t16067 * t3092 * t78496 * t6266 + 0.17149607247227894789e-2_f64 * t15618 * t23908 - t66721 / 216.0_f64 - 0.38110238327173099531e-3_f64 * t66763 - 0.12862205435420921092e-2_f64 * t3115 * t3117 * t89240 * t1045 - 0.85748036236139473944e-3_f64 * t3115 * t3117 * t89245 * t1045;
    (t89240, t89245, t89250)
}
