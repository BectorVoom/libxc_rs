//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 885/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk885(t1415: f64, t1646: f64, t42402: f64, t12990: f64, t7007: f64, t42085: f64, t550: f64, t30733: f64, t10122: f64, t2464: f64, t2465: f64, t587: f64) -> (f64, f64, f64, f64, f64) {
    let t42405 = 0.35750489951850426669e0_f64 * t1415 * t42402 * t1646;
    let t42407 = 0.71500979903700853338e0_f64 * t12990 * t7007;
    let t42408 = t550 * t42085;
    let t42412 = t12990 * t30733;
    let t42413 = 0.59584149919750711116e-1_f64 * t42412;
    let t42416 = t587 * t2464 * t2465 * t10122;
    (t42405, t42407, t42408, t42413, t42416)
}
