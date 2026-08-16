//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 965/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk965(t20173: f64, t31814: f64, t31817: f64, t1874: f64, t91854: f64, t23938: f64, t6525: f64, t1873: f64, t2311: f64, t2040: f64, t2314: f64, t31744: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114529 = 54.0_f64 * t20173 * t31814;
    let t114531 = 54.0_f64 * t20173 * t31817;
    let t114541 = 4.0_f64 * t91854 * t1874;
    let t114543 = 4.0_f64 * t23938 * t6525;
    let t114552 = t2311 * t1873;
    let t114554 = 2.0_f64 * t114552 * t2040;
    let t114559 = 4.0_f64 * t2314 * t31744;
    (t114529, t114531, t114541, t114543, t114552, t114554, t114559)
}
