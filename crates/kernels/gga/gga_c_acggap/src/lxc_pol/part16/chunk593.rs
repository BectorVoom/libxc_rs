//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 593/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk593<F: Float>(t1298: F, t513: F, t1089: F, t1095: F, t1524: F, t495: F, t1008: F, t1856: F, t418: F, t4275: F, t4279: F, t4280: F, t4285: F, t4288: F, t4308: F, t4310: F, t4312: F, t4320: F, t4322: F, t4324: F, t4328: F, t4339: F, t5529: F, t5534: F, t5539: F, t5542: F) -> (F, F, F, F, F) {
    let t5544 = t1298 * t513;
    let t5546 = t1089 * t1095 * t5544;
    let t5549 = t495 * t1524;
    let t5551 = t1089 * t1095 * t5549;
    let t5554 = t1008 * t1856;
    let t5558 = F::cast_from(0.25724410870841842184e-2_f64) * t418 * t5529 - F::cast_from(0.51448821741683684368e-2_f64) * t418 * t5534 + F::cast_from(0.25724410870841842184e-2_f64) * t418 * t5539 + F::cast_from(0.34299214494455789578e-2_f64) * t5542 + F::cast_from(0.34299214494455789578e-2_f64) * t418 * t5546 + F::cast_from(0.34299214494455789578e-2_f64) * t418 * t5551 - F::cast_from(0.17149607247227894789e-2_f64) * t5554 + t4275 - t4279 - F::cast_from(0.80031500487063509015e-2_f64) * t4280 - F::cast_from(0.85748036236139473945e-2_f64) * t4285 - t4288 + t4308 - t4310 + t4312 - t4320 + t4322 - t4324 + t4328 - t4339;
    (t5544, t5546, t5549, t5551, t5558)
}
