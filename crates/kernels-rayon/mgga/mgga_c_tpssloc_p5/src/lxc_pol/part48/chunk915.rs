//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 915/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk915(t23168: f64, t30678: f64, t23035: f64, t2379: f64, t30676: f64, t6637: f64, t30686: f64, t6579: f64, t1902: f64, t2631: f64, t1888: f64, t22996: f64, t2632: f64) -> (f64, f64, f64, f64, f64) {
    let t112968 = t23168 * t30678;
    let t112969 = 0.15352717957250113407e0_f64 * t112968;
    let t112973 = 0.9869604401089358619e-1_f64 * t23035 * t6637 * t30676 * t2379;
    let t112974 = t6579 * t30686;
    let t112975 = 0.76763589786250567036e-1_f64 * t112974;
    let t112976 = t1902 * t2631;
    let t112980 = 0.3289868133696452873e-1_f64 * t1888 * t22996 * t112976 * t2632;
    (t112969, t112973, t112975, t112976, t112980)
}
