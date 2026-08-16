//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 700/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk700(t13041: f64, t1445: f64, t833: f64, t2097: f64, t3039: f64, t3277: f64, t12658: f64, t3005: f64, t3295: f64, t9800: f64, t11053: f64, t9805: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13042 = t1445 * t13041;
    let t13044 = 0.43710935587469654631e2_f64 * t833 * t13042;
    let t13045 = t3039 * t2097;
    let t13047 = 0.25025342966295298669e1_f64 * t3277 * t13045;
    let t13050 = 0.11502877786176224903e1_f64 * t12658;
    let t13052 = t3005 * t3295;
    let t13053 = t9800 * t13052;
    let t13054 = 0.19171462976960374838e1_f64 * t13053;
    let t13055 = t11053 * t3295;
    let t13056 = t9805 * t13055;
    (t13042, t13044, t13045, t13047, t13050, t13052, t13054, t13055, t13056)
}
