//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1471/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1471(t1213: f64, t1216: f64, t248: f64, t45017: f64, t11862: f64, t1227: f64, t13969: f64, t11716: f64, t44833: f64, t44834: f64, t3503: f64, t1174: f64, t1197: f64, t2402: f64) -> (f64, f64, f64, f64, f64) {
    let t45020 = t1213 * t248 * t45017 * t1216;
    let t45027 = t1227 * t13969 * t11862;
    let t45030 = t44833 * t11716 * t44834;
    let t45037 = t44833 * t3503 * t44834;
    let t45044 = t1174 * t2402 * t1197;
    (t45020, t45027, t45030, t45037, t45044)
}
