//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1163/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1163(t34081: f64, t34091: f64, t34095: f64, t34099: f64, t34101: f64, t34107: f64, t30269: f64, t30273: f64, t30280: f64, t30297: f64, t34072: f64, t34074: f64, t34078: f64, t34085: f64, t34089: f64, t34105: f64, t34111: f64, t34115: f64) -> f64 {
    let t36934 = 0.31448092289604152068e-2_f64 * t34081;
    let t36937 = 0.34299214494455789578e-2_f64 * t34091;
    let t36938 = 0.12579236915841660828e-2_f64 * t34095;
    let t36939 = 0.42874018118069736972e-2_f64 * t34099;
    let t36940 = 0.21437009059034868486e-2_f64 * t34101;
    let t36942 = 0.18868855373762491241e-1_f64 * t34107;
    let t36945 = 0.18868855373762491241e-1_f64 * t30269 - 0.13719685797782315831e-1_f64 * t34072 + 0.68598428988911579156e-2_f64 * t34074 + 0.42874018118069736972e-3_f64 * t30273 + 0.57165357490759649296e-3_f64 * t30280 - 0.68598428988911579156e-2_f64 * t34078 - 0.42874018118069736972e-2_f64 * t30297 - t36934 - 0.31448092289604152068e-2_f64 * t34085 - 0.21437009059034868486e-2_f64 * t34089 + t36937 - t36938 - t36939 + t36940 - 0.18868855373762491241e-1_f64 * t34105 + t36942 + 0.21437009059034868486e-2_f64 * t34111 + 0.85748036236139473944e-3_f64 * t34115;
    t36945
}
