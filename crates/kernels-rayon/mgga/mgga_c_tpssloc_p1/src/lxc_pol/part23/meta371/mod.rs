//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1171;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta371(t3242: f64, t415: f64, t61: f64, t42341: f64, t44696: f64, t42344: f64, t483: f64, t1210: f64, t44722: f64, t478: f64, t11147: f64, t3439: f64, t11789: f64, t820: f64, t204: f64, t486: f64, t11716: f64, t3503: f64, t3584: f64, t676: f64, t221: f64, t44483: f64, t456: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44828, t44833, t44834, t44836, t44863, t44938) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1171(t3242, t415, t61, t42341, t44696, t42344, t483, t1210, t44722, t478, t11147, t3439);
        let (t44951, t45017, t45030, t45037, t45046, t45112) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1172(t11789, t820, t204, t486, t11716, t44833, t44834, t3503, t3584, t676, t221, t44483, t456);
    (t44828, t44836, t44863, t44938, t44951, t45017, t45030, t45037, t45046, t45112)
}
