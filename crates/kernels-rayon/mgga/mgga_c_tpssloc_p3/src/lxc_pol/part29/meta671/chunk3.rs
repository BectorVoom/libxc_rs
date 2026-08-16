//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2247/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2247(t16235: f64, t91361: f64, t5303: f64, t80820: f64, t16356: f64, t6916: f64, t16018: f64, t1998: f64, t236: f64, t6926: f64, t1339: f64, t54153: f64, t550: f64, t6936: f64) -> (f64, f64, f64, f64, f64) {
    let t91362 = t91361 * t16235;
    let t91364 = t80820 * t5303;
    let t91365 = 7.0_f64 / 288.0_f64 * t91364;
    let t91366 = t6916 * t16356;
    let t91370 = t6926 * t1998 * t236 * t16018;
    let t91374 = t6936 * t1339 * t54153 * t550;
    (t91362, t91365, t91366, t91370, t91374)
}
