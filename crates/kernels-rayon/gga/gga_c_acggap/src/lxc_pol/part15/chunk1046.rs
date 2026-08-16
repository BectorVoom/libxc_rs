//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1046/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1046(t34091: f64, t34095: f64, t34099: f64, t34101: f64, t34107: f64, t34130: f64, t34158: f64, t34170: f64, t34172: f64, t34175: f64, t34189: f64, t34204: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36937 = 0.34299214494455789578e-2_f64 * t34091;
    let t36938 = 0.12579236915841660828e-2_f64 * t34095;
    let t36939 = 0.42874018118069736972e-2_f64 * t34099;
    let t36940 = 0.21437009059034868486e-2_f64 * t34101;
    let t36942 = 0.18868855373762491241e-1_f64 * t34107;
    let t36951 = 0.42874018118069736972e-3_f64 * t34130;
    let t36962 = 0.13719685797782315831e-1_f64 * t34158;
    let t36967 = 0.21437009059034868486e-2_f64 * t34170;
    let t36968 = 0.13719685797782315831e-1_f64 * t34172;
    let t36969 = 0.21437009059034868486e-2_f64 * t34175;
    let t36972 = 0.12579236915841660827e-1_f64 * t34189;
    let t36976 = 0.16006300097412701803e-1_f64 * t34204;
    (t36937, t36938, t36939, t36940, t36942, t36951, t36962, t36967, t36968, t36969, t36972, t36976)
}
