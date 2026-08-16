//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1885/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1885(t23237: f64, t25341: f64, t6552: f64, t23204: f64, t25216: f64, t6562: f64, t1519: f64, t212: f64, t23171: f64, t6554: f64, t23270: f64, t25038: f64, t258: f64, t4119: f64, t776: f64) -> (f64, f64, f64, f64) {
    let t87907 = t6552 * t23237 * t25341;
    let t87910 = t6562 * t23204 * t25216;
    let t87915 = t23171 * t212 * t1519 * t6554;
    let t87920 = t25038 * t23270 * t258 * t4119 * t776;
    (t87907, t87910, t87915, t87920)
}
