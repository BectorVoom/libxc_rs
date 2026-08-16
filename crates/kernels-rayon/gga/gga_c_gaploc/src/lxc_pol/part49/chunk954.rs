//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 954/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk954(t12990: f64, t7007: f64, t30733: f64, t10122: f64, t2464: f64, t2465: f64, t587: f64, t27003: f64, t9438: f64, t12965: f64, t1407: f64, t41634: f64, t912: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42407 = 0.71500979903700853338e0_f64 * t12990 * t7007;
    let t42412 = t12990 * t30733;
    let t42413 = 0.59584149919750711116e-1_f64 * t42412;
    let t42416 = t587 * t2464 * t2465 * t10122;
    let t42420 = t587 * t9438 * t27003;
    let t42421 = 0.31952438294933958064e-1_f64 * t42420;
    let t42422 = t1407 * t12965;
    let t42425 = t587 * t912 * t41634;
    (t42407, t42413, t42416, t42421, t42422, t42425)
}
