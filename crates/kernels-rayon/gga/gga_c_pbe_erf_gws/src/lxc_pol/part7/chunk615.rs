//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 615/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk615(t50: f64, t1416: f64, t4367: f64, t4373: f64, t4767: f64, t4770: f64, t52: f64, t4766: f64, t59: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t4776 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t4767 * t4367 + 4.0_f64 / 3.0_f64 * t4770 * t1416 + 4.0_f64 / 3.0_f64 * t52 * t4373);
    let t4778 = (t4766 + t4776) * t59;
    t4778
}
