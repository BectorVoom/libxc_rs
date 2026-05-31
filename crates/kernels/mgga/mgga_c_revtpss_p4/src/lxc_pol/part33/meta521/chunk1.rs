//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1864/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1864<F: Float>(t2661: F, t27928: F, t25973: F, t25979: F, t25988: F, t25998: F, t26003: F, t26005: F, t26011: F, t26022: F, t26025: F, t27919: F, t27921: F, t27924: F, t27926: F) -> (F, F) {
    let t27929 = t2661 * t27928;
    let t27931 = -F::cast_from(0.25410001404642664113e-4_f64) * t25998 + t26003 + F::cast_from(0.20007875121765877254e-2_f64) * t26025 + t26022 - F::cast_from(0.10164000561857065645e-3_f64) * t25973 + F::cast_from(0.80031500487063509016e-2_f64) * t25979 + F::cast_from(0.14291339372689912324e-4_f64) * t25988 + F::cast_from(0.85748036236139473945e-2_f64) * t27919 + F::cast_from(0.20007875121765877254e-2_f64) * t27921 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t26005 - t26011 - F::cast_from(0.10164000561857065645e-3_f64) * t27924 + F::cast_from(0.80031500487063509015e-2_f64) * t27926 + F::cast_from(0.14291339372689912324e-4_f64) * t27929;
    (t27929, t27931)
}
