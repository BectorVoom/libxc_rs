//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1102/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1102(t2314: f64, t4193: f64, t847: f64, t3418: f64, t3422: f64, t4180: f64, t6669: f64, t4154: f64, t828: f64, t1359: f64, t3385: f64, t4170: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10802 = t4193 * t2314;
    let t10803 = t10802 * t847;
    let t10806 = t3422 * t3418;
    let t10809 = t4180 * t6669;
    let t10810 = t10809 * t847;
    let t10817 = t4154 * t828;
    let t10820 = t1359 * t3385;
    let t10823 = t4170 * t828;
    (t10802, t10803, t10806, t10809, t10810, t10817, t10820, t10823)
}
