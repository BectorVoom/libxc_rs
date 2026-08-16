//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1060/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1060(t36017: f64, t36030: f64, t36039: f64, t36041: f64, t36065: f64, t36081: f64, t36085: f64, t36087: f64, t36089: f64, t36096: f64, t36125: f64, t36131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37830 = 0.68598428988911579156e-2_f64 * t36017;
    let t37833 = 0.62896184579208304138e-3_f64 * t36030;
    let t37836 = 7.0_f64 / 12.0_f64 * t36039;
    let t37837 = 7.0_f64 / 36.0_f64 * t36041;
    let t37848 = 11.0_f64 / 144.0_f64 * t36065;
    let t37857 = 0.12579236915841660828e-2_f64 * t36081;
    let t37859 = 0.21437009059034868486e-2_f64 * t36085;
    let t37860 = 0.85748036236139473944e-3_f64 * t36087;
    let t37861 = 0.42874018118069736972e-3_f64 * t36089;
    let t37864 = 0.62896184579208304138e-3_f64 * t36096;
    let t37872 = 0.32012600194825403606e-1_f64 * t36125;
    let t37875 = 0.85748036236139473944e-3_f64 * t36131;
    (t37830, t37833, t37836, t37837, t37848, t37857, t37859, t37860, t37861, t37864, t37872, t37875)
}
