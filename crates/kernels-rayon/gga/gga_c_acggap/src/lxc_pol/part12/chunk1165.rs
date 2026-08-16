//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1165/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1165(t34170: f64, t34172: f64, t34175: f64, t34179: f64, t34189: f64, t34204: f64, t34217: f64, t30308: f64, t30310: f64, t30314: f64, t30316: f64, t30319: f64, t34183: f64, t34193: f64, t34197: f64, t34201: f64, t34208: f64, t34215: f64) -> f64 {
    let t36967 = 0.21437009059034868486e-2_f64 * t34170;
    let t36968 = 0.13719685797782315831e-1_f64 * t34172;
    let t36969 = 0.21437009059034868486e-2_f64 * t34175;
    let t36970 = 0.20965394859736101378e-2_f64 * t34179;
    let t36972 = 0.12579236915841660827e-1_f64 * t34189;
    let t36976 = 0.16006300097412701803e-1_f64 * t34204;
    let t36984 = 0.12579236915841660828e-2_f64 * t34217;
    let t36985 = -t36967 + t36968 + t36969 - t36970 - 0.94344276868812456208e-2_f64 * t34183 - t36972 + 0.37737710747524982482e-2_f64 * t34193 + 0.31448092289604152068e-2_f64 * t34197 - 0.47172138434406228102e-2_f64 * t34201 - t36976 - 0.75475421495049964966e-2_f64 * t34208 - 77.0_f64 / 144.0_f64 * t30308 - 77.0_f64 / 432.0_f64 * t30310 - 0.1528125e-1_f64 * t30314 - 0.62896184579208304138e-3_f64 * t30316 + 0.32012600194825403606e-1_f64 * t30319 - 0.62896184579208304138e-3_f64 * t34215 - t36984;
    t36985
}
