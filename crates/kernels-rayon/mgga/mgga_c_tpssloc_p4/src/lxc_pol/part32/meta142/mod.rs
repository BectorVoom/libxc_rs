//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta142(t1291: f64, t2663: f64, t1284: f64, t67: f64, t758: f64, t2225: f64, t522: f64, t2221: f64, t2223: f64, t2516: f64, t521: f64, t17: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3813, t3814, t3815, t3819, t3821, t3823, t3824, t3825) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk780(t1291, t2663, t1284, t67, t758, t2225, t522, t2221, t2223, t2516, t521, t17);
    (t3813, t3814, t3815, t3819, t3821, t3823, t3824, t3825)
}
