//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2189;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta606(t11687: f64, t11697: f64, t3577: f64, t11877: f64, t3576: f64, t11647: f64, t1203: f64, t11859: f64, t1222: f64, t11797: f64, t3490: f64, t11172: f64, t1227: f64, t248: f64, t3521: f64, t11801: f64, t204: f64, t486: f64, t1213: f64, t1216: f64, t11862: f64, t13969: f64, t11716: f64, t44833: f64, t44834: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44994, t44996, t45002, t45007, t45009, t45013) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2189(t11687, t11697, t3577, t11877, t3576, t11647, t1203, t11859, t1222, t11797, t3490, t11172, t1227, t248, t3521);
        let (t45015, t45017, t45020, t45027, t45030) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2190(t11801, t3490, t204, t486, t1213, t1216, t248, t11862, t1227, t13969, t11716, t44833, t44834);
    (t44994, t44996, t45002, t45007, t45009, t45013, t45015, t45017, t45020, t45027, t45030)
}
