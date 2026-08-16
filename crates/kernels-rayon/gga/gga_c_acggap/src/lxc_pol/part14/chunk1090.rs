//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1090/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1090(t7433: f64, t9641: f64, t1851: f64, t7614: f64, t30308: f64, t30310: f64, t30314: f64, t30319: f64, t34189: f64, t34204: f64, t34215: f64, t34218: f64, t34222: f64, t39080: f64, t39082: f64, t39086: f64, t39088: f64, t39092: f64, t39094: f64, t39098: f64) -> f64 {
    let t39100 = t7433 * t9641;
    let t39107 = t7614 * t1851;
    let t39109 = -0.85748036236139473944e-3_f64 * t39080 + 0.85748036236139473944e-3_f64 * t39082 - 0.62896184579208304134e-2_f64 * t34189 - 0.80031500487063509016e-2_f64 * t34204 - 0.85748036236139473944e-3_f64 * t39086 + 0.85748036236139473944e-3_f64 * t39088 - 0.7145669686344956162e-4_f64 * t39092 - 0.16006300097412701803e-1_f64 * t39094 + 0.47172138434406228102e-2_f64 * t39098 - 0.94344276868812456204e-3_f64 * t39100 - 77.0_f64 / 576.0_f64 * t30308 - 77.0_f64 / 1728.0_f64 * t30310 - 0.38203125e-2_f64 * t30314 + 0.80031500487063509016e-2_f64 * t30319 - 0.62896184579208304136e-3_f64 * t34215 - t34218 - t34222 - 0.32012600194825403606e-1_f64 * t39107;
    t39109
}
