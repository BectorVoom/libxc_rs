//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1253/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1253(t3616: f64, t7255: f64, t2649: f64, t9404: f64, t2757: f64, t3649: f64, t2754: f64, t2751: f64, t1057: f64, t9374: f64, t1134: f64, t9620: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26044 = t3616 * t7255;
    let t26046 = t9404 * t2649;
    let t26048 = t2757 * t3649;
    let t26050 = t2754 * t3649;
    let t26052 = t2751 * t3649;
    let t26054 = t1057 * t9374;
    let t26093 = t1134 * t9620;
    (t26044, t26046, t26048, t26050, t26052, t26054, t26093)
}
