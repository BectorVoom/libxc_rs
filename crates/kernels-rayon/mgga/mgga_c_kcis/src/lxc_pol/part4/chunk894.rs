//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 894/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk894(t2099: f64, t4425: f64, t1599: f64, t3978: f64, t617: f64, t5427: f64, t1610: f64, t1889: f64, t4440: f64, t1370: f64, t5441: f64, t1369: f64, t737: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6148 = t4425 * t2099;
    let t6149 = t1599 * t6148;
    let t6151 = t3978 * t617;
    let t6152 = t6151 * t5427;
    let t6155 = t1889 * t1610;
    let t6156 = t4440 * t6155;
    let t6159 = t1370 * t617;
    let t6160 = t6159 * t5441;
    let t6163 = t737 * t1369;
    (t6149, t6151, t6152, t6155, t6156, t6159, t6160, t6163)
}
