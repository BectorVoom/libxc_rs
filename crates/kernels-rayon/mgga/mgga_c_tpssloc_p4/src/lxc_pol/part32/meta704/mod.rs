//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2202;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta704(t24996: f64, t97890: f64, t28860: f64, t6876: f64, t1307: f64, t6324: f64, t22574: f64, t26162: f64, t28835: f64, t28830: f64, t24995: f64, t8643: f64, t74060: f64, t1388: f64, t1983: f64, t28238: f64, t6999: f64, t75214: f64, t12461: f64, t7752: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97892, t97893, t97897, t97899, t97905) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2202(t24996, t97890, t28860, t6876, t1307, t6324, t22574, t26162, t28835, t28830, t24995, t8643);
        let (t97910, t97914, t97916, t97919, t97920) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2203(t22574, t74060, t8643, t1388, t28830, t26162, t1983, t28238, t6999, t75214, t12461, t7752);
    (t97892, t97893, t97897, t97899, t97905, t97910, t97914, t97916, t97919, t97920)
}
