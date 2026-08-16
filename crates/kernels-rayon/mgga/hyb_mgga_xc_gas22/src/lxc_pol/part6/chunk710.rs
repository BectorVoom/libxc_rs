//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 710/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk710(t7: f64, t132: f64, t1112: f64, t3616: f64, t1179: f64, t2680: f64, t224: f64, t3: f64, t1793: f64, t545: f64, t1238: f64, t2688: f64, t341: f64, t675: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t3617 = t3616 * t1112;
    let t3619 = t2680 * t1179;
    let t3622 = t224 * t3;
    let t3626 = piecewise3(t8, 0.0_f64, 4.0_f64 / 9.0_f64 * t3619 * t545 + 8.0_f64 / 3.0_f64 * t3622 * t1793);
    let t3627 = t2688 * t1238;
    let t3630 = t341 * t3;
    let t3634 = piecewise3(t133, 0.0_f64, 4.0_f64 / 9.0_f64 * t3627 * t675 - 8.0_f64 / 3.0_f64 * t3630 * t1793);
    (t3617, t3619, t3622, t3626, t3627, t3630, t3634)
}
