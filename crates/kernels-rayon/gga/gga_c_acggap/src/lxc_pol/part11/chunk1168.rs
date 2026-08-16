//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1168/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1168(t35959: f64, t35962: f64, t35964: f64, t35965: f64, t35968: f64, t35969: f64, t35971: f64, t35973: f64, t35976: f64, t35978: f64, t35980: f64, t35982: f64, t35985: f64, t35988: f64, t35992: f64, t35995: f64, t35998: f64, t35999: f64) -> f64 {
    let t36001 = 0.85748036236139473944e-3_f64 * t35959 + t35962 + t35964 + 0.85748036236139473944e-3_f64 * t35965 - t35968 + 0.80031500487063509014e-2_f64 * t35969 - 0.85748036236139473944e-3_f64 * t35971 - 0.80031500487063509014e-2_f64 * t35973 - t35976 + t35978 - t35980 + t35982 + 0.7145669686344956162e-3_f64 * t35985 + t35988 + t35992 - 0.31448092289604152068e-2_f64 * t35995 - t35998 + 0.34299214494455789578e-2_f64 * t35999;
    t36001
}
