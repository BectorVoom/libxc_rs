//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2190/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2190(t11801: f64, t3490: f64, t204: f64, t486: f64, t1213: f64, t1216: f64, t248: f64, t11862: f64, t1227: f64, t13969: f64, t11716: f64, t44833: f64, t44834: f64) -> (f64, f64, f64, f64, f64) {
    let t45015 = t3490 * t11801;
    let t45017 = t204 * t486;
    let t45020 = t1213 * t248 * t45017 * t1216;
    let t45027 = t1227 * t13969 * t11862;
    let t45030 = t44833 * t11716 * t44834;
    (t45015, t45017, t45020, t45027, t45030)
}
