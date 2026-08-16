//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1085/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1085(t11947: f64, t11959: f64, t11974: f64, t11976: f64, t11979: f64, t11983: f64, t11986: f64, t11989: f64, t12002: f64, t12005: f64, t6597: f64, t9123: f64, t9142: f64) -> f64 {
    let t12159 = t9123 + t11947 - t6597 + t11959 + t11974 - t9142 - t11976 - t11979 - t11983 - t11986 + t11989 - t12002 - t12005;
    t12159
}
