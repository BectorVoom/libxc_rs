//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 450/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk450(t1952: f64, t551: f64, t553: f64, t536: f64, t547: f64, t331: f64, t535: f64, t1369: f64, t163: f64, t148: f64, t1371: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1955 = 0.65846301096364936273e-2_f64 * t1952 * t551 * t553;
    let t1958 = t536 * t547;
    let t1960 = t331 * t535;
    let t1962 = t1960 * t551 * t553;
    let t1964 = t1369 * t163;
    let t1966 = 0.31505407223141117834e-1_f64 * t148 * t1964;
    let t1969 = 0.39507780657818961764e-2_f64 * t550 * t1371 * t553;
    (t1955, t1958, t1960, t1962, t1964, t1966, t1969)
}
