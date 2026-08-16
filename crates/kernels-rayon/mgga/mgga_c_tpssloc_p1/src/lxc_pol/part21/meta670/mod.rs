//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2472;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta670(t11791: f64, t3490: f64, t11789: f64, t1227: f64, t248: f64, t3252: f64, t3248: f64, t11877: f64, t3576: f64, t11647: f64, t1203: f64, t204: f64, t486: f64, t1213: f64, t1216: f64, t11716: f64, t44833: f64, t44834: f64, t3503: f64, t1174: f64, t1197: f64, t2402: f64, t3584: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44968, t44972, t44976, t44996, t45002, t45017) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2472(t11791, t3490, t11789, t1227, t248, t3252, t3248, t11877, t3576, t11647, t1203, t204, t486);
        let (t45020, t45030, t45037, t45044, t45046) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2473(t1213, t1216, t248, t45017, t11716, t44833, t44834, t3503, t1174, t1197, t2402, t3584, t676);
    (t44968, t44972, t44976, t44996, t45002, t45017, t45020, t45030, t45037, t45044, t45046)
}
