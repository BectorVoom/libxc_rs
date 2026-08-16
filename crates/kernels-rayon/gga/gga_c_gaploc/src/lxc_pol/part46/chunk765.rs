//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 765/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk765(t2021: f64, t33575: f64, t10007: f64, t10627: f64, t32435: f64, t739: f64, t1: f64, t106: f64, t10667: f64, t316: f64, t11000: f64, t783: f64) -> (f64, f64, f64, f64, f64) {
    let t33576 = t2021 * t33575;
    let t33601 = t10007 * t10627;
    let t33680 = t739 * t32435;
    let t33725 = t10667 * t1 * t106 * t316;
    let t33778 = t11000 * t783;
    (t33576, t33601, t33680, t33725, t33778)
}
