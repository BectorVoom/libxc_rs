//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1371/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1371(t33248: f64, t33252: f64, t33254: f64, t33259: f64, t33263: f64, t33265: f64, t33270: f64, t33275: f64, t33278: f64, t33261: f64, t36520: f64, t33284: f64) -> (f64, f64) {
    let t36521 = 0.63350674672043801542e-5_f64 * t33248;
    let t36522 = 0.63350674672043801542e-5_f64 * t33252;
    let t36523 = 0.69504740211613770836e-3_f64 * t33254;
    let t36524 = 0.17207124582926432293e-7_f64 * t33259;
    let t36526 = 0.21720231316129303386e-4_f64 * t33263;
    let t36527 = 0.43440462632258606772e-4_f64 * t33265;
    let t36528 = 0.88465285289519332099e-6_f64 * t33270;
    let t36529 = 0.10117831965157323855e-7_f64 * t33275;
    let t36530 = 0.25002399603899953676e-2_f64 * t33278;
    let t36531 = -t36520 + t36521 + t36522 - t36523 - t36524 + 0.97817934710145362362e-6_f64 * t33261 + t36526 + t36527 + t36528 + t36529 + t36530;
    let t36534 = 0.75031332402051115813e-8_f64 * t33284;
    (t36531, t36534)
}
