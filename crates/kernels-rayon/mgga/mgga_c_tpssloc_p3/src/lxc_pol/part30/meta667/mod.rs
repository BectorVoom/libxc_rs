//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2093;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta667(t5259: f64, t80820: f64, t22779: f64, t26292: f64, t16060: f64, t6944: f64, t1827: f64, t80991: f64, t22765: f64, t5289: f64, t22764: f64, t5234: f64, t1354: f64, t26298: f64, t80958: f64, t26319: f64, t1358: f64, t26248: f64, t3862: f64, t7715: f64, t22705: f64, t22852: f64, t236: f64, t5286: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91215, t91226, t91278, t91282, t91284, t91285) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2093(t5259, t80820, t22779, t26292, t16060, t6944, t1827, t80991, t22765, t5289, t22764, t5234);
        let (t91287, t91290, t91301, t91304, t91305, t91310) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2094(t1354, t91285, t26298, t80958, t22779, t26319, t1358, t26248, t3862, t7715, t22705, t22852, t236, t5286, t550);
    (t91215, t91226, t91278, t91282, t91284, t91285, t91287, t91290, t91301, t91304, t91305, t91310)
}
