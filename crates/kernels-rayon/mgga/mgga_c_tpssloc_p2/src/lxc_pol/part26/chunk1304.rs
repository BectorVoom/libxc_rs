//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1304/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1304(t212: f64, t23171: f64, t6554: f64, t852: f64, t22986: f64, t23270: f64, t2717: f64, t2719: f64, t776: f64, t23030: f64, t23253: f64, t23204: f64, t23241: f64, t81640: f64) -> (f64, f64, f64, f64) {
    let t82087 = t23171 * t212 * t852 * t6554;
    let t82092 = t22986 * t23270 * t2717 * t2719 * t776;
    let t82099 = t23030 * t23253;
    let t82108 = t81640 * t23204 * t23241;
    (t82087, t82092, t82099, t82108)
}
