//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1162/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1162(t10055: f64, t2380: f64, t6475: f64, t2402: f64, t3860: f64, t2407: f64, t10258: f64, t8406: f64, t10266: f64, t2099: f64, t3235: f64, t10262: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28324 = t2380 * t6475 * t10055;
    let t28333 = t3860 * t2402;
    let t28335 = t3860 * t2407;
    let t28345 = t10258 * t8406;
    let t28353 = t3235 * t2099 * t10266;
    let t28364 = t3235 * t2099 * t10262;
    (t28324, t28333, t28335, t28345, t28353, t28364)
}
