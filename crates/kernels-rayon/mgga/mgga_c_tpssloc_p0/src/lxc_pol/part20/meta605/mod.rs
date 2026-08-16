//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2187;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta605(t1174: f64, t11760: f64, t135: f64, t11147: f64, t3439: f64, t11789: f64, t820: f64, t3577: f64, t3579: f64, t11737: f64, t44857: f64, t11791: f64, t3490: f64, t1227: f64, t248: f64, t3252: f64, t3248: f64, t11665: f64, t11698: f64, t11683: f64, t11697: f64, t11673: f64, t11678: f64, t11679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44936, t44938, t44951, t44953, t44965, t44968) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2187(t1174, t11760, t135, t11147, t3439, t11789, t820, t3577, t3579, t11737, t44857, t11791, t3490);
        let (t44972, t44976, t44982, t44985, t44988, t44991) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2188(t11789, t1227, t248, t3252, t3248, t11665, t11698, t11683, t11697, t3577, t11673, t11678, t11679);
    (t44936, t44938, t44951, t44953, t44965, t44968, t44972, t44976, t44982, t44985, t44988, t44991)
}
