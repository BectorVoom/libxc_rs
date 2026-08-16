//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1581;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1582;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta412(t22751: f64, t6970: f64, t3853: f64, t6945: f64, t3777: f64, t6944: f64, t1354: f64, t3787: f64, t59: f64, t240: f64, t1336: f64, t3795: f64, t6943: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22752, t22753, t22754, t22756, t22757, t22759, t22760, t22762) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1581(t22751, t6970, t3853, t6945, t3777, t6944, t1354, t3787, t59, t240, t1336, t3795);
        let (t22764, t22765) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1582(t6943, t835, t1336);
    (t22752, t22753, t22754, t22756, t22757, t22759, t22760, t22762, t22764, t22765)
}
