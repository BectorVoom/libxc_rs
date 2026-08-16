//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 947/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk947(t7: f64, t1793: f64, t545: f64, t1796: f64, t1808: f64, t3302: f64, t3305: f64, t461: f64, t776: f64, t8632: f64, t8635: f64, t222: f64, t37: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t8636 = t1793 * t545;
    let t8646 = piecewise3(t8, 0.0_f64, -28.0_f64 / 27.0_f64 * t8632 * t1808 + 16.0_f64 / 9.0_f64 * t8635 * t8636 + 4.0_f64 / 9.0_f64 * t3302 * t1796 - 2.0_f64 / 3.0_f64 * t776 * t1793 + 2.0_f64 * t3305 * t461);
    let t8648 = t222 * t37 * t8646;
    (t8636, t8646, t8648)
}
