//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1109/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1109(t132: f64, t3925: f64, t6975: f64, t2460: f64, t3938: f64, t10325: f64, t1794: f64, t3463: f64, t675: f64, t937: f64, t222: f64, t37: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t10900 = t6975 * t3925;
    let t10905 = t2460 * t3938;
    let t10911 = piecewise3(t133, 0.0_f64, -28.0_f64 / 27.0_f64 * t10900 * t675 - 16.0_f64 / 9.0_f64 * t3463 * t1794 + 4.0_f64 / 9.0_f64 * t10905 * t675 - t937 * t10325 / 3.0_f64);
    let t10913 = t222 * t37 * t10911;
    (t10900, t10905, t10911, t10913)
}
