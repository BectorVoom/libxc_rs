//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 798/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk798(t27003: f64, t587: f64, t9438: f64, t12965: f64, t1407: f64, t41634: f64, t912: f64, t2487: f64, t27007: f64, t26328: f64, t6914: f64, t1365: f64, t31558: f64, t6525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42420 = t587 * t9438 * t27003;
    let t42422 = t1407 * t12965;
    let t42425 = t587 * t912 * t41634;
    let t42428 = t2487 * t9438 * t27007;
    let t42431 = t6914 * t9438 * t26328;
    let t42529 = t6525 * t1365 * t31558;
    (t42420, t42422, t42425, t42428, t42431, t42529)
}
