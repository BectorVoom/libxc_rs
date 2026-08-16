//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 917/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk917(t8004: f64, t4652: f64, t4664: f64, t4746: f64, t4751: f64, t4784: f64, t4790: f64, t4799: f64, t7994: f64, t7995: f64, t7997: f64, t7999: f64, t8000: f64, t8001: f64, t8002: f64, t8003: f64) -> (f64, f64) {
    let t8005 = 0.24415406715670879921e-3_f64 * t8004;
    let t8006 = t4746 + t4751 + t4652 - t7994 - t7995 + t4664 + t7997 + t7999 - t4784 - t8000 - t4790 - t8001 - t8002 + t8003 + t8005 - t4799;
    (t8005, t8006)
}
