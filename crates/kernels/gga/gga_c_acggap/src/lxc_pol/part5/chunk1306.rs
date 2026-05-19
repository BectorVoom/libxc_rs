//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1306/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1306<F: Float>(t3409: F, t5864: F, t3382: F, t6153: F, t5534: F, t997: F, t5539: F, t1017: F, t1165: F, t1180: F, t13287: F, t15565: F, t17413: F, t1759: F, t17656: F, t1782: F, t17912: F, t1844: F, t18832: F, t18839: F, t18841: F, t22778: F, t397: F, t398: F, t418: F, t4298: F, t525: F, t5989: F, t6151: F, t8401: F, t966: F) -> F {
    let t24302 = t3409 * t5864;
    let t24304 = t3382 * t6153;
    let t24320 = t997 * t5534;
    let t24331 = t997 * t5539;
    let t24333 = F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1165 * t15565 * t1759 - F::cast_from(0.25724410870841842183e-2_f64) * t18832 + F::cast_from(0.40015750243531754508e-2_f64) * t24302 + F::cast_from(0.17149607247227894789e-2_f64) * t24304 + F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t1165 * t4298 * t6151 - F::cast_from(0.34299214494455789578e-2_f64) * t18839 - F::cast_from(0.34299214494455789577e-2_f64) * t18841 - F::cast_from(0.10289764348336736873e-1_f64) * t17656 * t17912 * t525 * t22778 + F::cast_from(0.34299214494455789578e-2_f64) * t17656 * t13287 * t8401 * t5989 + F::cast_from(0.4801890029223810541e-1_f64) * t24320 + F::cast_from(0.25724410870841842184e-1_f64) * t418 * t398 * t17413 * t1782 * t1017 - F::cast_from(0.42874018118069736972e-3_f64) * t397 * t398 * t966 * t1844 - F::cast_from(0.24009450146119052706e-1_f64) * t24331;
    t24333
}
