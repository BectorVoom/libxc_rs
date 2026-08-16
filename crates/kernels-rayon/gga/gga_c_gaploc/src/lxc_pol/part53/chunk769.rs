//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 769/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk769(t1: f64, t106: f64, t5745: f64, t787: f64, t191: f64, t5750: f64, t2925: f64, t5241: f64, t10627: f64, t22623: f64, t24885: f64, t1457: f64, t2634: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32809 = t787 * t5745 * t1 * t106;
    let t32810 = t191 * t5750;
    let t32840 = t5241 * t2925;
    let t32847 = t22623 * t10627;
    let t32969 = t787 * t24885;
    let t32970 = t1457 * t2634;
    (t32809, t32810, t32840, t32847, t32969, t32970)
}
