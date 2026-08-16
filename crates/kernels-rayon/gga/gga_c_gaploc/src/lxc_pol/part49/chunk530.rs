//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 530/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk530(t105: f64, t1358: f64, t9062: f64, t9067: f64, t9072: f64, t9077: f64, t9080: f64, t9085: f64, t9089: f64, t9092: f64, t9094: f64, t9130: f64, t9158: f64, t9203: f64, t9239: f64) -> f64 {
    let t9241 = -0.31616674039640166221e-2_f64 * t1358 * t9062 - 0.31616674039640166221e-2_f64 * t1358 * t9067 - t9072 + t9077 + 0.94850022118920498663e-2_f64 * t1358 * t9080 + t9085 - t9089 + t9092 - t9094 + 0.28455006635676149599e-1_f64 * t105 * t9130 + t9158 + t9203 + t9239;
    t9241
}
