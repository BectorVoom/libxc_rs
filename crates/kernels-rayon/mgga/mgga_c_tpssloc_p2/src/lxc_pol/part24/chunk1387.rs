//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1387/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1387(t1015: f64, t23472: f64, t23503: f64, t10423: f64, t23419: f64, t23418: f64, t3180: f64, t10401: f64, t23417: f64, t3186: f64, t3158: f64, t6712: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82996 = t23472 * t1015 * t23503;
    let t83004 = t23419 * t10423;
    let t83008 = t3180 * t23418;
    let t83015 = t23417 * t10401;
    let t83016 = t3186 * t83015;
    let t83025 = t6712 * t3158;
    (t82996, t83004, t83008, t83015, t83016, t83025)
}
