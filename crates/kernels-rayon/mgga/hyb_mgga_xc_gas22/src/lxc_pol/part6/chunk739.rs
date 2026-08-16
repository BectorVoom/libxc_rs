//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 739/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk739(t3814: f64, t559: f64, t1179: f64, t2997: f64, t3804: f64, t1193: f64) -> (f64, f64, f64, f64) {
    let t3815 = t559 * t3814;
    let t3819 = t2997 * t1179;
    let t3823 = t559 * t3804;
    let t3827 = t1193 * t1193;
    (t3815, t3819, t3823, t3827)
}
