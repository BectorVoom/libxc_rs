//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 667/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk667(t12156: f64, t12191: f64, t12199: f64, t12202: f64, t12235: f64, t12247: f64, t12249: f64, t12267: f64, t3749: f64, t841: f64, t3730: f64, t747: f64) -> (f64, f64, f64) {
    let t12270 = t12156 + t12191 + t12199 + t12202 + t12235 + t12247 + t12249 + t12267;
    let t12272 = t3749 * t841;
    let t12277 = t3730 * t747;
    (t12270, t12272, t12277)
}
