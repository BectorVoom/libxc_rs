//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2327/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2327(t18356: f64, t24729: f64, t27614: f64, t4997: f64, t1730: f64, t27603: f64, t27598: f64, t5001: f64, t1218: f64, t1232: f64, t1737: f64, t18523: f64, t19101: f64, t2134: f64, t24736: f64, t460: f64, t5014: f64, t6211: f64, t6227: f64, t7320: f64, t7345: f64, t86140: f64, t95238: f64, t95507: f64, t95511: f64, t95512: f64) -> f64 {
    let t104294 = t24729 * t18356;
    let t104296 = t27614 * t4997;
    let t104300 = t1730 * t27603;
    let t104303 = t5001 * t27598;
    let t104319 = t104294 / 1152.0_f64 + t104296 / 1152.0_f64 + t86140 * t6227 / 768.0_f64 + t104300 * t1232 / 216.0_f64 - t104303 * t1218 / 144.0_f64 + t95238 * t1737 / 768.0_f64 + t27614 * t5014 / 768.0_f64 - t7345 * t19101 / 2304.0_f64 - t24736 * t6211 / 1152.0_f64 - 0.10093189023535097714e-3_f64 * t2134 * t18523 * t460 * t7320 - t95507 + t95511 + t95512 / 648.0_f64;
    t104319
}
