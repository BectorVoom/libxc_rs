//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 897/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk897(t10: f64, t16: f64, t2212: f64, t1806: f64, t92: f64, t125: f64, t3: f64, t545: f64, t13: f64, t2969: f64, t6209: f64) -> (f64, f64, f64, f64, f64) {
    let t7831 = t2212 * t10 * t16;
    let t7834 = t2212 * t1806;
    let t7835 = t7834 * t92;
    let t7836 = t125 * t3;
    let t7837 = t7836 * t545;
    let t7842 = t6209 * t13 * t2969;
    (t7831, t7834, t7835, t7837, t7842)
}
