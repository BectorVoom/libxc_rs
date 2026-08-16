//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 701/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk701(t1368: f64, t3051: f64, t1969: f64, t3052: f64, t5900: f64, t1017: f64, t590: f64, t2185: f64, t23657: f64, t1039: f64, t558: f64, t23608: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27142 = t1368 * t3051;
    let t27144 = t1969 * t5900 * t3052;
    let t27145 = t27142 * t27144;
    let t27147 = t1017 * t590;
    let t27149 = t2185 * t5900 * t27147;
    let t27150 = t23657 * t27149;
    let t27152 = t1039 * t558;
    let t27154 = t2185 * t5900 * t27152;
    let t27155 = t23657 * t27154;
    let t27157 = t23608 * t92;
    (t27142, t27145, t27147, t27150, t27152, t27155, t27157)
}
