//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1371/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1371(t3490: f64, t2484: f64, t2496: f64, t21462: f64, t2485: f64, t4247: f64, t4251: f64, t7009: f64, t21474: f64, t7025: f64, t10887: f64, t2490: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29824 = t3490 * t3490;
    let t29825 = t2484 * t29824;
    let t29827 = t2496 * t29824;
    let t29833 = t21462 * t4247 * t2485;
    let t29836 = t7009 * t4251 * t2485;
    let t29839 = t21474 * t4247 * t2485;
    let t29842 = t7025 * t4251 * t2485;
    let t29844 = t10887 * t2490;
    (t29825, t29827, t29833, t29836, t29839, t29842, t29844)
}
