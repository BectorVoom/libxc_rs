//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 472/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk472(t164: f64, t762: f64, t1597: f64, t528: f64, t547: f64, t147: f64, t837: f64, t551: f64, t553: f64, t536: f64, t331: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1947 = 0.63010814446282235668e-1_f64 * t762 * t164;
    let t1948 = t1597 * t164;
    let t1951 = 0.63010814446282235668e-1_f64 * t528 * t547;
    let t1952 = t837 * t147;
    let t1955 = 0.65846301096364936273e-2_f64 * t1952 * t551 * t553;
    let t1958 = t536 * t547;
    let t1960 = t331 * t535;
    (t1947, t1948, t1951, t1952, t1955, t1958, t1960)
}
