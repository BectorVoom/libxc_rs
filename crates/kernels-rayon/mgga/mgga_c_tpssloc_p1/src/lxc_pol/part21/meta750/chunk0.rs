//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2622/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2622(t12339: f64, t5314: f64, t1831: f64, t40059: f64, t16336: f64, t3872: f64, t16060: f64, t3865: f64, t1369: f64, t16123: f64, t68: f64, t1362: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53897 = t12339 * t5314;
    let t53901 = t40059 * t1831;
    let t53903 = t16336 * t3872;
    let t53906 = t16060 * t3865;
    let t53907 = t53906 * t1369;
    let t53909 = t16123 * t68;
    let t53910 = t53909 * t1362;
    (t53897, t53901, t53903, t53906, t53907, t53909, t53910)
}
