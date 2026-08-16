//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1286/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1286(t7: f64, t27034: f64, t27096: f64, t27635: f64, t27684: f64, t27706: f64, t27757: f64, t27797: f64, t27843: f64, t10199: f64, t2028: f64, t10191: f64, t3138: f64, t8498: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t27847 = piecewise3(t9, 0.0_f64, t27034 + t27096 + t27635 + t27684 + t27706 + t27757 + t27797 + t27843);
    let t27852 = t10199 * t2028;
    let t27857 = t3138 * t8498 * t10191;
    (t27847, t27852, t27857)
}
