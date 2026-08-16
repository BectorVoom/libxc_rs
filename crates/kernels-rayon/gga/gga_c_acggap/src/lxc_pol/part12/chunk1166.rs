//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1166/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1166(t34221: f64, t34237: f64, t34239: f64, t34255: f64, t34271: f64, t34223: f64, t34225: f64, t34227: f64, t34229: f64, t34233: f64, t34242: f64, t34246: f64, t34251: f64, t34259: f64, t34263: f64, t34265: f64, t34269: f64, t34273: f64) -> f64 {
    let t36987 = 0.12579236915841660828e-2_f64 * t34221;
    let t36993 = 0.42874018118069736972e-3_f64 * t34237;
    let t36994 = 0.34299214494455789578e-2_f64 * t34239;
    let t36998 = 0.85748036236139473944e-3_f64 * t34255;
    let t37003 = 0.17149607247227894789e-2_f64 * t34271;
    let t37005 = -t36987 + 0.13719685797782315831e-1_f64 * t34223 - 0.68598428988911579156e-2_f64 * t34225 + 0.10289764348336736873e-1_f64 * t34227 + 0.51448821741683684367e-2_f64 * t34229 - 0.37737710747524982483e-2_f64 * t34233 + t36993 - t36994 + 0.62896184579208304138e-3_f64 * t34242 + 0.18868855373762491241e-2_f64 * t34246 + 0.18868855373762491242e-1_f64 * t34251 - t36998 + 0.18868855373762491241e-1_f64 * t34259 - 0.12579236915841660828e-2_f64 * t34263 + 0.17149607247227894789e-2_f64 * t34265 - 0.75475421495049964964e-2_f64 * t34269 - t37003 - 0.80031500487063509014e-2_f64 * t34273;
    t37005
}
