//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2073;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta656(t46104: f64, t6489: f64, t12571: f64, t22522: f64, t26083: f64, t9239: f64, t645: f64, t7445: f64, t1863: f64, t22550: f64, t7441: f64, t9231: f64, t2240: f64, t26043: f64, t33: f64, t45844: f64, t111: f64, t26097: f64, t26351: f64, t6883: f64, t22751: f64, t26186: f64, t26190: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90182, t90185, t90192, t90248, t90251, t90308) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2073(t46104, t6489, t12571, t22522, t26083, t9239, t645, t7445, t1863, t22550, t7441, t9231);
        let (t90312, t90330, t90400, t90460, t90469, t90470) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2074(t2240, t26043, t33, t45844, t6489, t111, t26097, t26351, t6883, t22751, t26186, t26190);
    (t90182, t90185, t90192, t90248, t90251, t90308, t90312, t90330, t90400, t90460, t90469, t90470)
}
