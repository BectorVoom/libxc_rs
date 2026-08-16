//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1154/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1154(t17: f64, t7768: f64, t2850: f64, t412: f64, t11406: f64, t3957: f64, t126: f64, t19: f64, t8184: f64, t547: f64, t5888: f64, t2986: f64, t641: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15052 = t7768 * t17;
    let t15681 = t2850 * t412;
    let t15686 = t11406 * t3957;
    let t19557 = 5.0_f64 / 108.0_f64 * t19 * t8184 * t126;
    let t19568 = t547 * t5888;
    let t19571 = t19 * t2986 * t641;
    (t15052, t15681, t15686, t19557, t19568, t19571)
}
