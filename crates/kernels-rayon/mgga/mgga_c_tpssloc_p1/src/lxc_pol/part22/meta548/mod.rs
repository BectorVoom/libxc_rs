//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2046;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta548(t116: f64, t1314: f64, t9534: f64, t1307: f64, t133: f64, t6600: f64, t59: f64, t9223: f64, t120: f64, t212: f64, t22815: f64, t67: f64, t535: f64, t1317: f64, t40005: f64, t9580: f64, t3741: f64, t2566: f64, t3732: f64, t12214: f64, t792: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40369, t40372, t40394, t40399) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2046(t116, t1314, t9534, t1307, t133, t6600, t59, t9223, t120, t212, t22815, t67);
        let (t40401, t40402, t40406, t40407, t40409, t40412) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2047(t40394, t40399, t535, t1317, t40005, t1314, t9580, t3741, t2566, t3732, t12214, t792);
    (t40369, t40372, t40394, t40399, t40401, t40402, t40406, t40407, t40409, t40412)
}
