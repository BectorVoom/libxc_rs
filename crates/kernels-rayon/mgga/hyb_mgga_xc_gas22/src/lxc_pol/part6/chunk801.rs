//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 801/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk801(t7: f64, t132: f64, t4407: f64, t4457: f64, t224: f64, t2680: f64, t3804: f64, t3814: f64, t2688: f64, t341: f64, t3925: f64, t3938: f64, t259: f64, zeta_threshold: f64) -> (f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t4458 = t4407 + t4457;
    let t4467 = piecewise3(t8, 0.0_f64, 4.0_f64 / 9.0_f64 * t2680 * t3814 + 4.0_f64 / 3.0_f64 * t224 * t3804);
    let t4473 = piecewise3(t133, 0.0_f64, 4.0_f64 / 9.0_f64 * t2688 * t3925 + 4.0_f64 / 3.0_f64 * t341 * t3938);
    let t4475 = (t4467 + t4473) * t259;
    (t4458, t4475)
}
