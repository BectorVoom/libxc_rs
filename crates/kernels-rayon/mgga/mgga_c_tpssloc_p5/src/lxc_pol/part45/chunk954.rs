//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 954/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk954(t114107: f64, t1992: f64, t550: f64, t6976: f64, t22897: f64, t3792: f64, t31207: f64, t6883: f64, t22724: f64, t31198: f64, t22704: f64, t22705: f64, t31202: f64) -> (f64, f64, f64, f64, f64) {
    let t114111 = 0.16449340668482264365e-1_f64 * t1992 * t6976 * t114107 * t550;
    let t114115 = 0.3289868133696452873e-1_f64 * t1992 * t22897 * t114107 * t3792;
    let t114116 = t6883 * t31207;
    let t114117 = 0.76763589786250567036e-1_f64 * t114116;
    let t114119 = 0.52089578783527170489e-1_f64 * t22724 * t31198;
    let t114121 = t22704 * t22705 * t31202;
    (t114111, t114115, t114117, t114119, t114121)
}
