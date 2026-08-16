//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 812/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk812(t1314: f64, t142: f64, t8806: f64, t1318: f64, t7436: f64, t2313: f64, t361: f64, t2030: f64, t1298: f64, t599: f64, t2317: f64, t2060: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8807 = t142 * t1314;
    let t8808 = t8806 * t8807;
    let t8810 = t142 * t1318;
    let t8811 = t7436 * t8810;
    let t8813 = t361 * t2313;
    let t8814 = t2030 * t8813;
    let t8816 = t599 * t1298;
    let t8817 = t142 * t8816;
    let t8818 = t2030 * t8817;
    let t8820 = t361 * t2317;
    let t8821 = t2060 * t8820;
    (t8807, t8808, t8810, t8811, t8813, t8814, t8816, t8817, t8818, t8820, t8821)
}
