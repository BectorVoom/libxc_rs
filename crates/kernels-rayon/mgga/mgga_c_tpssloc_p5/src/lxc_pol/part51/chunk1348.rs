//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1348/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1348(t120544: f64, t6888: f64, t6891: f64, t114299: f64, t114285: f64, t26331: f64, t26333: f64, t114316: f64, t32769: f64, t6883: f64, t1985: f64, t26193: f64, t31123: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120616 = 0.3289868133696452873e-1_f64 * t6888 * t120544 * t6891;
    let t120621 = 0.82246703342411321825e-2_f64 * t114299;
    let t120628 = 0.9869604401089358619e-1_f64 * t26331 * t114285 * t26333;
    let t120629 = 0.16449340668482264365e-1_f64 * t114316;
    let t120632 = t6883 * t32769;
    let t120633 = 0.38381794893125283518e-1_f64 * t120632;
    let t120641 = 0.16449340668482264365e-1_f64 * t1985 * t26193 * t31123;
    (t120616, t120621, t120628, t120629, t120633, t120641)
}
