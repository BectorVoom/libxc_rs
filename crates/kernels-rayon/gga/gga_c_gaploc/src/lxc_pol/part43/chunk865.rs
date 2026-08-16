//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 865/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk865(t42420: f64, t2487: f64, t27007: f64, t9438: f64, t26328: f64, t6914: f64, t12891: f64, t1580: f64, t1445: f64, t3085: f64, t597: f64, t7995: f64) -> (f64, f64, f64, f64, f64) {
    let t42421 = 0.31952438294933958064e-1_f64 * t42420;
    let t42428 = t2487 * t9438 * t27007;
    let t42429 = 0.7988109573733489516e-1_f64 * t42428;
    let t42431 = t6914 * t9438 * t26328;
    let t42432 = 0.47928657442400937096e-1_f64 * t42431;
    let t42438 = 0.43710935587469654631e2_f64 * t1580 * t12891;
    let t42442 = 0.43710935587469654631e2_f64 * t597 * t1445 * t7995 * t3085;
    (t42421, t42429, t42432, t42438, t42442)
}
