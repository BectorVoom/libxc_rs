//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1188/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1188(t35145: f64, t35148: f64, t30949: f64, t30956: f64, t30963: f64, t30967: f64, t30974: f64, t30976: f64, t30978: f64, t30980: f64, t30982: f64, t30985: f64, t30987: f64, t35139: f64, t35151: f64, t35154: f64, t35157: f64) -> f64 {
    let t37408 = 7.0_f64 / 36.0_f64 * t35145;
    let t37409 = 7.0_f64 / 36.0_f64 * t35148;
    let t37419 = -0.32012600194825403606e-1_f64 * t30949 - 0.42874018118069736972e-3_f64 * t35139 + 0.42874018118069736972e-3_f64 * t30956 + 0.85748036236139473944e-3_f64 * t30963 - 0.14291339372689912324e-3_f64 * t30967 + 0.31448092289604152069e-3_f64 * t30974 - t37408 - t37409 + t35151 / 12.0_f64 + t35154 / 12.0_f64 + t35157 / 12.0_f64 - 0.64025200389650807212e-1_f64 * t30976 + 0.32012600194825403606e-1_f64 * t30978 + 0.32012600194825403606e-1_f64 * t30980 - 0.32012600194825403606e-1_f64 * t30982 + 0.42874018118069736972e-2_f64 * t30985 - 0.51448821741683684368e-2_f64 * t30987;
    t37419
}
