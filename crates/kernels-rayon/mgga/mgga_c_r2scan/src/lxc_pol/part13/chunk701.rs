//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 701/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk701(t1800: f64, t652: f64, t621: f64, t5380: f64, t188: f64, t1891: f64, t1893: f64, t230: f64, t4982: f64, t152: f64, t1724: f64, t158: f64) -> (f64, f64, f64, f64, f64) {
    let t5381 = t652 * t1800;
    let t5382 = t5381 * t621;
    let t5384 = 0.2894756309764656312e3_f64 * t5380 * t5382;
    let t5388 = t1891 * t188;
    let t5389 = t1893 * t1800;
    let t5390 = t5389 * t621;
    let t5392 = 0.1551780387578202009e4_f64 * t5388 * t5390;
    let t5393 = t4982 * t230;
    let t5396 = 1.0_f64 / t1724 / t152;
    let t5397 = t5396 * t158;
    (t5381, t5384, t5392, t5393, t5397)
}
