//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1821;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1822;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta501(t4028: f64, t6534: f64, t1458: f64, t649: f64, t1873: f64, t4072: f64, t88: f64, t7676: f64, t2314: f64, t7467: f64, t5113: f64, t1453: f64, t22470: f64, t666: f64, t109: f64, t22473: f64, t4067: f64, t6530: f64, t22469: f64, t22471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26113, t26114) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1821(t4028, t6534, t1458, t649);
        let (t26116, t26117, t26119, t26121, t26123, t26125, t26127, t26129) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1822(t1873, t26114, t4072, t88, t6534, t7676, t2314, t7467, t5113, t1453, t22470, t666);
        let t26135 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1823(t109, t22473, t26129, t4067, t6530, t22469, t22471, t26127);
    (t26113, t26114, t26116, t26117, t26119, t26121, t26123, t26125, t26127, t26129, t26135)
}
