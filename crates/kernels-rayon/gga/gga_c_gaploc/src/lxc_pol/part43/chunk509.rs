//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 509/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk509(t9285: f64, t9287: f64, t1: f64, t9078: f64, t544: f64, t2365: f64, t6520: f64, t7025: f64, t9060: f64, t9065: f64, t1415: f64, t2371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9289 = 0.29792074959875355558e-1_f64 * t9285 * t9287;
    let t9290 = t9078 * t1;
    let t9291 = t544 * t9290;
    let t9294 = t2365 * t6520;
    let t9296 = 0.29792074959875355558e-1_f64 * t7025 * t9294;
    let t9297 = t9060 * t1;
    let t9298 = t544 * t9297;
    let t9301 = t9065 * t1;
    let t9302 = t544 * t9301;
    let t9305 = t1415 * t2371;
    (t9289, t9290, t9291, t9294, t9296, t9297, t9298, t9301, t9302, t9305)
}
