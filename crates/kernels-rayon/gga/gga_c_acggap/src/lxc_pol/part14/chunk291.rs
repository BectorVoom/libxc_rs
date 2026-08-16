//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 291/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk291(t1327: f64, t960: f64, t1000: f64, t1002: f64, t1007: f64, t1009: f64, t1011: f64, t1150: f64, t1315: f64, t1319: f64, t1324: f64, t335: f64, t367: f64, t936: f64, t953: f64, t976: f64, t979: f64, t983: f64, t989: f64, t995: f64, t998: f64) -> (f64, f64) {
    let t1328 = t960 * t1327;
    let t1336 = -0.21437009059034868486e-3_f64 * t936 + 0.10003937560882938627e-2_f64 * t953 + t976 - t979 + t983 + t1150 * t1315 / 16.0_f64 + t335 * t1319 / 48.0_f64 + t335 * t1324 / 48.0_f64 + t367 * t1328 / 48.0_f64 + t989 - t995 + 0.40015750243531754508e-2_f64 * t998 - 0.20007875121765877254e-2_f64 * t1000 + 0.20007875121765877254e-2_f64 * t1002 - t1007 - 0.85748036236139473944e-3_f64 * t1009 + 0.42874018118069736972e-3_f64 * t1011;
    (t1328, t1336)
}
