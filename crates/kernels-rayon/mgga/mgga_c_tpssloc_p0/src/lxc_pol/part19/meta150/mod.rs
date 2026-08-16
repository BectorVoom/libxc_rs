//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk759;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk760;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta150(t193: f64, t533: f64, t131: f64, t3732: f64, t205: f64, t242: f64, t3788: f64, t1336: f64, t557: f64, t67: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t5160, t5194, t5195, t5245, t5246) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk759(t193, t533, t131, t3732, t205, t242, t3788, t1336);
        let t5248 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk760(t557, t67, t246);
    (t5160, t5194, t5195, t5245, t5246, t5248)
}
