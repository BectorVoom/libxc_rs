//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2472/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2472(t11791: f64, t3490: f64, t11789: f64, t1227: f64, t248: f64, t3252: f64, t3248: f64, t11877: f64, t3576: f64, t11647: f64, t1203: f64, t204: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44968 = t3490 * t11791;
    let t44972 = t1227 * t248 * t11789 * t3252;
    let t44976 = t1227 * t248 * t11789 * t3248;
    let t44996 = t11877 * t3576;
    let t45002 = t1203 * t11647;
    let t45017 = t204 * t486;
    (t44968, t44972, t44976, t44996, t45002, t45017)
}
