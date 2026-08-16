//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3660/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3660(t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64) -> f64 {
    let t69167 = -0.27397333333333333332e0_f64 * t68363 + 0.761037037037037037e-1_f64 * t68366 - 0.20294320987654320986e-1_f64 * t56176 + 0.6088296296296296296e-1_f64 * t56183 - 0.4566222222222222222e-1_f64 * t56185 - 0.2283111111111111111e-1_f64 * t56187 - 0.6849333333333333333e-1_f64 * t56189 + 0.1522074074074074074e-1_f64 * t56209 + 0.761037037037037037e-2_f64 * t56212 + 0.4566222222222222222e-1_f64 * t56214 - 0.12683950617283950617e-1_f64 * t56216 + 0.3044148148148148148e-1_f64 * t56228;
    t69167
}
