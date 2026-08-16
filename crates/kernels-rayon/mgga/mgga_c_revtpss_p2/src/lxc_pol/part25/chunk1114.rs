//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1114/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1114(t25242: f64, t2482: f64, t27: f64, t7036: f64, t2487: f64, t2479: f64, t7045: f64, t2648: f64, t7038: f64, t2689: f64, t7030: f64, t1945: f64, t2693: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25243 = 0.90357964994909313586e-5_f64 * t25242;
    let t25245 = t2482 * t7036 * t27;
    let t25246 = t25245 * t2487;
    let t25248 = t7045 * t2479;
    let t25251 = t7038 * t2648;
    let t25253 = t2689 * t7030;
    let t25254 = 0.15244095330869239812e-3_f64 * t25253;
    let t25255 = t1945 * t2693;
    (t25243, t25245, t25246, t25248, t25251, t25254, t25255)
}
