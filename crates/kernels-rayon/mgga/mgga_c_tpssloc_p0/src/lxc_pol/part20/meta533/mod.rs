//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2069;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta533(t12226: f64, t16094: f64, t3719: f64, t686: f64, t3736: f64, t40018: f64, t59: f64, t9223: f64, t116: f64, t120: f64, t212: f64, t22815: f64, t67: f64, t535: f64, t1317: f64, t40005: f64, t12189: f64, t3745: f64, t1314: f64, t9580: f64, t3741: f64, t2566: f64, t3732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40376, t40387, t40394, t40399) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2069(t12226, t16094, t3719, t686, t3736, t40018, t59, t9223, t116, t120, t212, t22815, t67);
        let (t40401, t40402, t40404, t40406, t40407, t40409) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2070(t40394, t40399, t535, t1317, t40005, t12189, t3745, t1314, t9580, t3741, t2566, t3732);
    (t40376, t40387, t40394, t40399, t40401, t40402, t40404, t40406, t40407, t40409)
}
