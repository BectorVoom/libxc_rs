//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1306/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1306(t3409: f64, t5864: f64, t3382: f64, t6153: f64, t5534: f64, t997: f64, t5539: f64, t1017: f64, t1165: f64, t1180: f64, t13287: f64, t15565: f64, t17413: f64, t1759: f64, t17656: f64, t1782: f64, t17912: f64, t1844: f64, t18832: f64, t18839: f64, t18841: f64, t22778: f64, t397: f64, t398: f64, t418: f64, t4298: f64, t525: f64, t5989: f64, t6151: f64, t8401: f64, t966: f64) -> f64 {
    let t24302 = t3409 * t5864;
    let t24304 = t3382 * t6153;
    let t24320 = t997 * t5534;
    let t24331 = t997 * t5539;
    let t24333 = 0.85748036236139473944e-3_f64 * t1180 * t1165 * t15565 * t1759 - 0.25724410870841842183e-2_f64 * t18832 + 0.40015750243531754508e-2_f64 * t24302 + 0.17149607247227894789e-2_f64 * t24304 + 0.17149607247227894789e-2_f64 * t1180 * t1165 * t4298 * t6151 - 0.34299214494455789578e-2_f64 * t18839 - 0.34299214494455789577e-2_f64 * t18841 - 0.10289764348336736873e-1_f64 * t17656 * t17912 * t525 * t22778 + 0.34299214494455789578e-2_f64 * t17656 * t13287 * t8401 * t5989 + 0.4801890029223810541e-1_f64 * t24320 + 0.25724410870841842184e-1_f64 * t418 * t398 * t17413 * t1782 * t1017 - 0.42874018118069736972e-3_f64 * t397 * t398 * t966 * t1844 - 0.24009450146119052706e-1_f64 * t24331;
    t24333
}
