//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1088/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1088(t1941: f64, t243: f64, t2712: f64, t64: f64, t2710: f64, t826: f64, t2482: f64, t27: f64, t7036: f64, t2689: f64, t7030: f64, t2718: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25237 = t1941 * t243;
    let t25240 = t64 * t2712;
    let t25242 = t2710 * t25240 * t826;
    let t25243 = 0.90357964994909313586e-5_f64 * t25242;
    let t25245 = t2482 * t7036 * t27;
    let t25253 = t2689 * t7030;
    let t25254 = 0.15244095330869239812e-3_f64 * t25253;
    let t25260 = t2718 * t64;
    (t25237, t25240, t25243, t25245, t25254, t25260)
}
