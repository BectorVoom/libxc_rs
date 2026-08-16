//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 745/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk745(t418: f64, t4275: f64, t4279: f64, t4280: f64, t4285: f64, t4288: f64, t4308: f64, t4310: f64, t4312: f64, t4320: f64, t4322: f64, t4324: f64, t4328: f64, t4339: f64, t5529: f64, t5534: f64, t5539: f64, t5542: f64, t5546: f64, t5551: f64, t5554: f64) -> f64 {
    let t5558 = 0.25724410870841842184e-2_f64 * t418 * t5529 - 0.51448821741683684368e-2_f64 * t418 * t5534 + 0.25724410870841842184e-2_f64 * t418 * t5539 + 0.34299214494455789578e-2_f64 * t5542 + 0.34299214494455789578e-2_f64 * t418 * t5546 + 0.34299214494455789578e-2_f64 * t418 * t5551 - 0.17149607247227894789e-2_f64 * t5554 + t4275 - t4279 - 0.80031500487063509015e-2_f64 * t4280 - 0.85748036236139473945e-2_f64 * t4285 - t4288 + t4308 - t4310 + t4312 - t4320 + t4322 - t4324 + t4328 - t4339;
    t5558
}
