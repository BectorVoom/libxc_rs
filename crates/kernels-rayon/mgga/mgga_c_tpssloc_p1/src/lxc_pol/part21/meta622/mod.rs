//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2400;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta622(t212: f64, t2586: f64, t3734: f64, t40353: f64, t12225: f64, t3719: f64, t116: f64, t1314: f64, t9534: f64, t1307: f64, t133: f64, t6600: f64, t3736: f64, t40018: f64, t59: f64, t9223: f64, t120: f64, t22815: f64, t67: f64, t535: f64, t1317: f64, t40005: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40356, t40360, t40369, t40372) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2400(t212, t2586, t3734, t40353, t12225, t3719, t116, t1314, t9534, t1307, t133, t6600);
        let (t40387, t40394, t40399, t40401, t40402) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2401(t3736, t40018, t59, t9223, t116, t120, t212, t22815, t67, t535, t1317, t40005);
    (t40356, t40360, t40369, t40372, t40387, t40394, t40399, t40401, t40402)
}
