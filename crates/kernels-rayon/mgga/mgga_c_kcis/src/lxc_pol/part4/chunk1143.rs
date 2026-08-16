//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1143/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1143(t1662: f64, t3040: f64, t2894: f64, t2909: f64, t4972: f64, t1003: f64, t417: f64, t1245: f64, t4967: f64, t991: f64, t1704: f64, t2911: f64, t9874: f64) -> (f64, f64, f64, f64) {
    let t14501 = t1662 * t3040;
    let t14502 = t2894 * t14501;
    let t14511 = t2909 * t4972;
    let t14512 = t14511 * t1003;
    let t14513 = t417 * t14512;
    let t14516 = t1245 * t4967;
    let t14518 = t991 * t14516 / 72.0_f64;
    let t14522 = t9874 * t1704 * t2911;
    (t14502, t14513, t14518, t14522)
}
