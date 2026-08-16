//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1056/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1056(t35623: f64, t35631: f64, t35646: f64, t35672: f64, t35678: f64, t35682: f64, t35685: f64, t35702: f64, t35709: f64, t35736: f64, t35747: f64, t35755: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37636 = 0.12579236915841660828e-2_f64 * t35623;
    let t37639 = 0.18868855373762491241e-2_f64 * t35631;
    let t37646 = 0.305625e-1_f64 * t35646;
    let t37658 = 0.13719685797782315831e-1_f64 * t35672;
    let t37661 = 0.13719685797782315831e-1_f64 * t35678;
    let t37663 = 0.57165357490759649296e-3_f64 * t35682;
    let t37665 = 11.0_f64 / 24.0_f64 * t35685;
    let t37672 = 0.18868855373762491241e-2_f64 * t35702;
    let t37675 = 0.64025200389650807212e-1_f64 * t35709;
    let t37696 = 0.68598428988911579156e-2_f64 * t35736;
    let t37701 = 0.85748036236139473944e-3_f64 * t35747;
    let t37704 = 0.34299214494455789578e-1_f64 * t35755;
    (t37636, t37639, t37646, t37658, t37661, t37663, t37665, t37672, t37675, t37696, t37701, t37704)
}
