//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 964/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk964(t1016: f64, t39340: f64, t12148: f64, t1382: f64, t14295: f64, t4349: f64, t605: f64, t1022: f64, t3720: f64, t12256: f64, t1991: f64, t43363: f64, t44090: f64, t45170: f64, t45174: f64, t45176: f64, t45177: f64, t45178: f64, t45179: f64, t45180: f64, t45183: f64, t45187: f64, t45188: f64, t45192: f64, t45193: f64, t45194: f64, t45195: f64, t47140: f64, t47145: f64, t590: f64, t739: f64) -> (f64, f64, f64, f64, f64) {
    let t49977 = 2.0_f64 * t39340 * t1016;
    let t49980 = 4.0_f64 * t1382 * t1016 * t12148;
    let t49983 = 12.0_f64 * t4349 * t14295 * t605;
    let t49989 = t1022 * t3720;
    let t49998 = t45170 - t45174 - 0.38342925953920749676e1_f64 * t43363 - t45176 - t45177 - t45178 + t45179 - t45180 - t45183 + 0.20449560508757733161e1_f64 * t1991 * t739 * t49989 * t590 - 0.14300195980740170668e1_f64 * t12256 * t44090 + t45187 + t45188 + t45192 - t45193 - t45194 + t45195 + 0.10224780254378866581e1_f64 * t47140 + 0.76685851907841499354e0_f64 * t47145;
    (t49977, t49980, t49983, t49989, t49998)
}
