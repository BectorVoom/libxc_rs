//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 750/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk750(t1478: f64, t95: f64, t194: f64, t417: f64, t1305: f64, t201: f64, t1297: f64, t212: f64, tau1: f64) -> (f64, f64, f64, f64, f64) {
    let t3961 = t95 * t1478;
    let t3962 = t417 * t194;
    let t3963 = 1.0_f64 / t3962;
    let t3965 = 1.0_f64 / t1305 / t201;
    let t3969 = t1297 * t212;
    let t3972 = tau1 * tau1;
    (t3961, t3963, t3965, t3969, t3972)
}
