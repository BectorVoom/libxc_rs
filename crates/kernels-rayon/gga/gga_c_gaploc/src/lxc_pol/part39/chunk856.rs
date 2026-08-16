//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 856/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk856(t12161: f64, t795: f64, t1853: f64, t3721: f64, t12380: f64, t455: f64, t145: f64, t459: f64, t12385: f64, t2281: f64, t1246: f64, t135: f64, t4074: f64, t458: f64, t9105: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t39403 = t795 * t12161;
    let t39454 = t3721 * t1853;
    let t39622 = t12380 * t455;
    let t39624 = t39622 * t145 * t459;
    let t39626 = t2281 * t12385;
    let t39632 = t9105 * t4074 * pi * t1246 * t135 * t458;
    (t39403, t39454, t39622, t39624, t39626, t39632)
}
