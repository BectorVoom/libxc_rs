//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 957/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk957(t114160: f64, t6888: f64, t6891: f64, t2006: f64, t794: f64, t6897: f64, t6907: f64, t22724: f64, t31127: f64, t80645: f64, t8458: f64, t1985: f64, t22934: f64, t31137: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114171 = 0.6579736267392905746e-1_f64 * t6888 * t114160 * t6891;
    let t114172 = t794 * t2006;
    let t114174 = t6897 * t114172 * t6907;
    let t114175 = 0.16449340668482264365e-1_f64 * t114174;
    let t114178 = 0.52089578783527170489e-1_f64 * t22724 * t31127;
    let t114187 = t6897 * t80645 * t8458;
    let t114188 = 0.16449340668482264365e-1_f64 * t114187;
    let t114193 = 0.3289868133696452873e-1_f64 * t1985 * t31137 * t22934;
    (t114171, t114172, t114175, t114178, t114188, t114193)
}
