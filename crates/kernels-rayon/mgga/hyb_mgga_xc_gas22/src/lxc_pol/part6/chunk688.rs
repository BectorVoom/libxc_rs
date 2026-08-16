//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 688/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk688(t132: f64, t1238: f64, t2460: f64, t3: f64, t937: f64, t1793: f64, t675: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t3463 = t2460 * t1238;
    let t3466 = t937 * t3;
    let t3470 = piecewise3(t133, 0.0_f64, 4.0_f64 / 9.0_f64 * t3463 * t675 + 2.0_f64 / 3.0_f64 * t3466 * t1793);
    (t3463, t3466, t3470)
}
