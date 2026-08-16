//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1205/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1205(t332: f64, t4408: f64, t13869: f64, t13972: f64, t13949: f64, t14001: f64, t13957: f64, t14113: f64, t2222: f64, t3955: f64, t13953: f64, t13976: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51922 = t4408 * t332;
    let t51928 = t13972 * t13869;
    let t51952 = t14001 * t13949;
    let t51954 = t14113 * t13957;
    let t51958 = t3955 * t2222;
    let t51960 = t13953 * t13976;
    (t51922, t51928, t51952, t51954, t51958, t51960)
}
