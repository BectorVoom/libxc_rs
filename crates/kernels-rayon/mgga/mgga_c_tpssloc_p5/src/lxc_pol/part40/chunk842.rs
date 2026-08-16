//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 842/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk842(t6168: f64, t68: f64, t484: f64, t3560: f64, t5392: f64, t974: f64, t1196: f64, t5398: f64, t3555: f64, t1653: f64, t1735: f64, t3578: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6169 = t6168 * t68;
    let t6170 = t6169 * t484;
    let t6177 = t3560 * t5392;
    let t6178 = t974 * t6177;
    let t6183 = t1196 * t5398;
    let t6184 = t974 * t6183;
    let t6187 = t3555 * t5392;
    let t6188 = t974 * t6187;
    let t6191 = t1735 * t1653;
    let t6192 = t3578 * t6191;
    (t6169, t6170, t6177, t6178, t6183, t6184, t6187, t6188, t6191, t6192)
}
