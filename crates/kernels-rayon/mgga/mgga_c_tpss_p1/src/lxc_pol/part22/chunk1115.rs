//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1115/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1115(t1501: f64, t3074: f64, t3068: f64, t1098: f64, t12279: f64, t12290: f64, t12294: f64, t12295: f64, t12298: f64, t12301: f64, t12304: f64, t12307: f64, t12310: f64, t12319: f64, t12321: f64, t3067: f64, t3103: f64, t3107: f64, t4265: f64, t9526: f64, t9530: f64, t9535: f64, t9538: f64, t9543: f64, t9547: f64) -> f64 {
    let t12324 = t1501 * t3074;
    let t12325 = t3068 * t12324;
    let t12328 = -7.0_f64 / 648.0_f64 * t1098 * t12279 + 5.0_f64 / 20736.0_f64 * t9526 - t9530 / 4608.0_f64 + t9535 + t9538 / 4608.0_f64 - t9543 / 6912.0_f64 + t9547 / 2304.0_f64 + t12290 - t12294 + t1098 * t12295 / 108.0_f64 + t1098 * t12298 / 216.0_f64 + t1098 * t12301 / 36.0_f64 - t1098 * t12304 / 72.0_f64 - t1098 * t12307 / 144.0_f64 - t1098 * t12310 / 48.0_f64 + t4265 * t3107 / 864.0_f64 + t4265 * t3103 / 432.0_f64 - t12319 - t3067 * t12321 / 2304.0_f64 - t3067 * t12325 / 4608.0_f64;
    t12328
}
