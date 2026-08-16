//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1101/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1101(t2039: f64, t671: f64, t1401: f64, t3938: f64, t3941: f64, t577: f64, t7056: f64, t7222: f64, t7230: f64, t1184: f64, t460: f64, t33: f64, t3953: f64) -> (f64, f64, f64, f64) {
    let t7235 = t2039 * t671;
    let t7240 = 0.45e1_f64 * t7222 * t577 + 0.135e2_f64 * t7230 * t671 + 0.135e2_f64 * t3938 * t2039 + 27.0_f64 * t3941 * t7235 + 0.135e2_f64 * t1401 * t7056;
    let t7319 = t1184 * t460;
    let t7428 = t3953 * t33;
    (t7235, t7240, t7319, t7428)
}
