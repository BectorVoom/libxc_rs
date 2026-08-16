//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2191;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta607(t3503: f64, t44833: f64, t44834: f64, t1174: f64, t1197: f64, t2402: f64, t3584: f64, t676: f64, t1227: f64, t248: f64, t3243: f64, t1011: f64, t1212: f64, t44706: f64, t11692: f64, t11693: f64, t11697: f64, t11853: f64, t1213: f64, t3570: f64, t11163: f64, t3521: f64, t221: f64, t44483: f64, t456: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45037, t45044, t45049, t45080) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2191(t3503, t44833, t44834, t1174, t1197, t2402, t3584, t676, t1227, t248, t3243, t1011, t1212, t44706);
        let (t45086, t45102, t45108, t45112) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2192(t11692, t11693, t11697, t11853, t1213, t248, t3570, t11163, t1227, t3521, t221, t44483, t456);
    (t45037, t45044, t45049, t45080, t45086, t45102, t45108, t45112)
}
