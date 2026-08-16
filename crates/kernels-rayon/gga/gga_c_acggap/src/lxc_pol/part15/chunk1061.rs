//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1061/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1061(t36133: f64, t36151: f64, t36156: f64, t36162: f64, t36175: f64, t36177: f64, t36194: f64, t36198: f64, t36205: f64, t36207: f64, t36210: f64, t36214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37876 = 0.85748036236139473944e-3_f64 * t36133;
    let t37888 = 7.0_f64 / 72.0_f64 * t36151;
    let t37892 = 0.12579236915841660828e-2_f64 * t36156;
    let t37894 = 0.85748036236139473944e-3_f64 * t36162;
    let t37898 = 0.18868855373762491241e-2_f64 * t36175;
    let t37899 = 0.68598428988911579156e-2_f64 * t36177;
    let t37904 = 0.62896184579208304136e-2_f64 * t36194;
    let t37905 = 0.94344276868812456204e-2_f64 * t36198;
    let t37907 = 0.36675e0_f64 * t36205;
    let t37908 = 0.183375e0_f64 * t36207;
    let t37909 = 0.183375e0_f64 * t36210;
    let t37910 = 0.183375e0_f64 * t36214;
    (t37876, t37888, t37892, t37894, t37898, t37899, t37904, t37905, t37907, t37908, t37909, t37910)
}
