//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 888/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk888(t42433: f64, t4820: f64, t6824: f64, t12891: f64, t1580: f64, t1445: f64, t3085: f64, t597: f64, t7995: f64, t11392: f64, t3159: f64, t10348: f64, t10485: f64) -> (f64, f64, f64, f64, f64) {
    let t42435 = t6824 * t4820 * t42433;
    let t42438 = 0.43710935587469654631e2_f64 * t1580 * t12891;
    let t42442 = 0.43710935587469654631e2_f64 * t597 * t1445 * t7995 * t3085;
    let t42444 = 0.25025342966295298669e1_f64 * t3159 * t11392;
    let t42448 = t10485 * t10348;
    (t42435, t42438, t42442, t42444, t42448)
}
