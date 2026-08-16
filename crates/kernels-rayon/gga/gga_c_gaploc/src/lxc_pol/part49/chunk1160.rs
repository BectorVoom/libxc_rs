//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1160/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1160(t1897: f64, t39454: f64, t954: f64, t2508: f64, t47130: f64, t688: f64, t779: f64, t12213: f64, t2580: f64, t7291: f64, t12218: f64, t7226: f64) -> (f64, f64, f64, f64) {
    let t47673 = t1897 * t954 * t39454;
    let t47677 = t2508 * t779 * t47130 * t688;
    let t47681 = t2508 * t2580 * t12213 * t7291;
    let t47685 = t2508 * t7226 * t12218 * t7291;
    (t47673, t47677, t47681, t47685)
}
