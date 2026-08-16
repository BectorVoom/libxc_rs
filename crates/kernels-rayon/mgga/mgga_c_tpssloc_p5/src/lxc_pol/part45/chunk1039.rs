//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1039/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1039(t1873: f64, t91854: f64, t23938: f64, t6534: f64, t91857: f64, t26977: f64, t22479: f64, t7042: f64, t2319: f64, t8518: f64, t2307: f64, t8513: f64, t8514: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t115813 = 4.0_f64 * t91854 * t1873;
    let t115815 = 4.0_f64 * t23938 * t6534;
    let t115817 = 2.0_f64 * t91857 * t1873;
    let t115819 = 4.0_f64 * t26977 * t6534;
    let t115821 = 2.0_f64 * t7042 * t22479;
    let t115824 = t8518 * t2319;
    let t115829 = t8513 * t8514 * t2307;
    (t115813, t115815, t115817, t115819, t115821, t115824, t115829)
}
