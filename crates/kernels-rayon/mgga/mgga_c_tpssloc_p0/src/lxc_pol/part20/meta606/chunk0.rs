//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2189/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2189(t11687: f64, t11697: f64, t3577: f64, t11877: f64, t3576: f64, t11647: f64, t1203: f64, t11859: f64, t1222: f64, t11797: f64, t3490: f64, t11172: f64, t1227: f64, t248: f64, t3521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44994 = t3577 * t11697 * t11687;
    let t44996 = t11877 * t3576;
    let t45002 = t1203 * t11647;
    let t45007 = t11859 * t1222;
    let t45009 = t3490 * t11797;
    let t45013 = t1227 * t248 * t3521 * t11172;
    (t44994, t44996, t45002, t45007, t45009, t45013)
}
