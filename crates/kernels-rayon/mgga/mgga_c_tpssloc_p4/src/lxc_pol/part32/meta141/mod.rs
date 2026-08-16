//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta141 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk777;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk778;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta141(t1339: f64, t835: f64, t1336: f64, t1354: f64, t242: f64, t1365: f64, t67: f64, t246: f64, t1307: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3798, t3799) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk777(t1339, t835, t1336);
        let (t3800, t3802, t3803, t3804, t3805) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk778(t1354, t3799, t1339, t242, t1336, t1365, t67, t246);
        let t3807 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk779(t1307, t550);
    (t3798, t3799, t3800, t3802, t3803, t3804, t3805, t3807)
}
