//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1888;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta444(t11285: f64, t3377: f64, t14853: f64, t1164: f64, t300: f64, t4832: f64, t1166: f64, t3419: f64, t4869: f64, t11180: f64, t1671: f64, t3259: f64, t4782: f64, t14704: f64, t14710: f64, t14722: f64, t11215: f64, t11217: f64, t14720: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64, t14766: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14854, t14855, t14857, t14858, t14860, t14862, t14864, t14866) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1888(t11285, t3377, t14853, t1164, t300, t4832, t1166, t3419, t4869, t11180, t1671, t3259, t4782);
        let (t14868, t14870, t14887) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1889(t14704, t14710, t14722, t11215, t11217, t14720, t14733, t14738, t14742, t14746, t14751, t14755, t14766);
    (t14854, t14855, t14857, t14858, t14860, t14862, t14864, t14866, t14868, t14870, t14887)
}
