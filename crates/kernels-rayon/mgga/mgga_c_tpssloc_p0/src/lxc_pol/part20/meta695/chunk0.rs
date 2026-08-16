//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2647/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2647(t16060: f64, t3865: f64, t1369: f64, t16123: f64, t68: f64, t1362: f64, t1831: f64, t40292: f64, t12345: f64, t5314: f64, t12211: f64, t16296: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53906 = t16060 * t3865;
    let t53907 = t53906 * t1369;
    let t53909 = t16123 * t68;
    let t53910 = t53909 * t1362;
    let t53917 = t40292 * t1831;
    let t53918 = 119.0_f64 / 1152.0_f64 * t53917;
    let t53919 = t12345 * t5314;
    let t53920 = 119.0_f64 / 1152.0_f64 * t53919;
    let t53921 = t12211 * t16296;
    (t53907, t53909, t53910, t53918, t53920, t53921)
}
