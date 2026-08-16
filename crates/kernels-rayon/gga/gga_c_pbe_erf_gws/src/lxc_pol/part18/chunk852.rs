//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 852/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk852(t2936: f64, t751: f64, t1: f64, t1098: f64, t2057: f64, t2062: f64, t1167: f64, t6854: f64, t1161: f64, t6781: f64, t829: f64, t830: f64) -> (f64, f64, f64, f64, f64) {
    let t8503 = t751 * t2936;
    let t8519 = t1098 * t2057 * t1;
    let t8520 = t8519 * t2062;
    let t8546 = t1167 * t6854;
    let t8582 = t6781 * t1161;
    let t8584 = t829 * t830 * t8582;
    (t8503, t8520, t8546, t8582, t8584)
}
