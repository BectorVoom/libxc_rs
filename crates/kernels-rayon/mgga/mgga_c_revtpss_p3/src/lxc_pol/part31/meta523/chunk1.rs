//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1889/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1889(t2661: f64, t27928: f64, t25973: f64, t25979: f64, t25988: f64, t25998: f64, t26003: f64, t26005: f64, t26011: f64, t26022: f64, t26025: f64, t27919: f64, t27921: f64, t27924: f64, t27926: f64) -> (f64, f64) {
    let t27929 = t2661 * t27928;
    let t27931 = -0.25410001404642664113e-4_f64 * t25998 + t26003 + 0.20007875121765877254e-2_f64 * t26025 + t26022 - 0.10164000561857065645e-3_f64 * t25973 + 0.80031500487063509016e-2_f64 * t25979 + 0.14291339372689912324e-4_f64 * t25988 + 0.85748036236139473945e-2_f64 * t27919 + 0.20007875121765877254e-2_f64 * t27921 + 7.0_f64 / 144.0_f64 * t26005 - t26011 - 0.10164000561857065645e-3_f64 * t27924 + 0.80031500487063509015e-2_f64 * t27926 + 0.14291339372689912324e-4_f64 * t27929;
    (t27929, t27931)
}
