//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1212/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1212(t13869: f64, t13972: f64, t13881: f64, t840: f64, t4052: f64, t6781: f64, t829: f64, t830: f64, t13949: f64, t14001: f64, t13957: f64, t14113: f64) -> (f64, f64, f64, f64, f64) {
    let t51928 = t13972 * t13869;
    let t51930 = t840 * t13881;
    let t51945 = t6781 * t4052;
    let t51947 = t829 * t830 * t51945;
    let t51952 = t14001 * t13949;
    let t51954 = t14113 * t13957;
    (t51928, t51930, t51947, t51952, t51954)
}
