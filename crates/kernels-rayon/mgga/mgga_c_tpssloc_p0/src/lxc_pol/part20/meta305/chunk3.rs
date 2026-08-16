//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1552/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1552(t241: f64, t3439: f64, t11148: f64, t136: f64, t11154: f64, t3297: f64, t11161: f64, t11170: f64, t11195: f64, t11197: f64, t11200: f64, t11204: f64, t11206: f64, t11209: f64, t11211: f64, t11213: f64, t11215: f64, t11217: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11219 = t241 * t3439;
    let t11220 = t11219 * t11148;
    let t11221 = t136 * t11220;
    let t11223 = t3297 * t11154;
    let t11224 = t136 * t11223;
    let t11228 = -t11195 - 0.28483875e1_f64 * t11197 + 0.46074375e0_f64 * t11200 - t11204 + 0.49293999999999999999e0_f64 * t11206 + 0.82156666666666666667e-1_f64 * t11209 + 0.27385555555555555556e0_f64 * t11211 + 0.5477111111111111111e-1_f64 * t11213 - 0.32862666666666666666e0_f64 * t11215 - 0.16431333333333333333e0_f64 * t11217 + 0.36514074074074074075e-1_f64 * t11221 - 0.16431333333333333333e0_f64 * t11224 - 0.59793333333333333333e0_f64 * t11161 + 0.17938e1_f64 * t11170;
    (t11219, t11220, t11221, t11223, t11224, t11228)
}
