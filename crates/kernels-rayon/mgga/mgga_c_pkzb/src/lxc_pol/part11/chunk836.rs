//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 836/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk836(t2593: f64, t2639: f64, t179: f64, t3403: f64, t5221: f64, t1702: f64, t3407: f64, t3402: f64, t568: f64, t581: f64, t1024: f64, t2575: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8920 = t2593 * t2639;
    let t8921 = t179 * t8920;
    let t8924 = t5221 * t3403;
    let t8926 = t1702 * t3407;
    let t8931 = t581 * t3402 * t568;
    let t8935 = t581 * t1024 * t2575;
    (t8920, t8921, t8924, t8926, t8931, t8935)
}
