//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 782/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk782<F: Float>(t213: F, t7910: F, t5629: F, t7271: F, t1885: F, t26024: F, t25972: F, t5622: F, t1889: F, t25978: F, t25986: F, t5609: F, t2661: F, t25973: F, t25979: F, t25988: F, t25998: F, t26003: F, t26005: F, t26011: F, t26022: F, t26025: F) -> (F, F) {
    let t27909 = t213 * t7910;
    let t27919 = t7271 * t5629;
    let t27921 = t26024 * t1885;
    let t27924 = t25972 * t5622;
    let t27926 = t25978 * t1889;
    let t27928 = t25986 * t5609;
    let t27929 = t2661 * t27928;
    let t27931 = -0.25410001404642664113e-4 * t25998 + t26003 + 0.20007875121765877254e-2 * t26025 + t26022 - 0.10164000561857065645e-3 * t25973 + 0.80031500487063509016e-2 * t25979 + 0.14291339372689912324e-4 * t25988 + 0.85748036236139473945e-2 * t27919 + 0.20007875121765877254e-2 * t27921 + 7.0 / 144.0 * t26005 - t26011 - 0.10164000561857065645e-3 * t27924 + 0.80031500487063509015e-2 * t27926 + 0.14291339372689912324e-4 * t27929;
    (t27909, t27931)
}
