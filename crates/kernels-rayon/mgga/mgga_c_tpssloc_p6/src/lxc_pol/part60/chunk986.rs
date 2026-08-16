//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 986/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk986(t1992: f64, t550: f64, t6976: f64, t97181: f64, t120437: f64, t1825: f64, t22633: f64, t120514: f64, t120521: f64, t97172: f64, t22897: f64, t3792: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127391 = 0.16449340668482264365e-1_f64 * t1992 * t6976 * t97181 * t550;
    let t127402 = 0.6579736267392905746e-1_f64 * t22633 * t6976 * t120437 * t1825;
    let t127403 = 0.76763589786250567036e-1_f64 * t120514;
    let t127404 = 0.16449340668482264365e-1_f64 * t120521;
    let t127408 = 0.16449340668482264365e-1_f64 * t1992 * t6976 * t97172 * t550;
    let t127412 = 0.3289868133696452873e-1_f64 * t1992 * t22897 * t97172 * t3792;
    (t127391, t127402, t127403, t127404, t127408, t127412)
}
