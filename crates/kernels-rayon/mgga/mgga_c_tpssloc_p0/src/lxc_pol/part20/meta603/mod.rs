//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2183;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta603(t1174: f64, t11765: f64, t135: f64, t3551: f64, t698: f64, t3242: f64, t415: f64, t42341: f64, t44696: f64, t42344: f64, t483: f64, t1210: f64, t3561: f64, t11738: f64, t11739: f64, t248: f64, t3570: f64, t10471: f64, t44690: f64, t11727: f64, t44722: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44803, t44811, t44827, t44833, t44834, t44836) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2183(t1174, t11765, t135, t3551, t698, t3242, t415, t42341, t44696, t42344, t483, t1210);
        let (t44847, t44851, t44857, t44858, t44863) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2184(t1174, t3561, t698, t11738, t11739, t248, t3570, t10471, t44690, t11727, t44722, t44833, t44834, t478);
    (t44803, t44811, t44827, t44833, t44834, t44836, t44847, t44851, t44857, t44858, t44863)
}
