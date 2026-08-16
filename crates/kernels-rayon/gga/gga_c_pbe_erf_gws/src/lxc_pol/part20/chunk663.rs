//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 663/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk663(t43: f64, t50: f64, t3629: f64, t3631: f64, t3633: f64, t3635: f64, zeta_threshold: f64) -> f64 {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t3711 = piecewise3(t44, 0.0_f64, -2.0_f64 / 9.0_f64 * t3629 + 2.0_f64 / 3.0_f64 * t3631);
    let t3715 = piecewise3(t51, 0.0_f64, -2.0_f64 / 9.0_f64 * t3633 + 2.0_f64 / 3.0_f64 * t3635);
    let t3717 = t3711 / 2.0_f64 + t3715 / 2.0_f64;
    t3717
}
