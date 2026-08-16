//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta199 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta199(t300: f64, t4479: f64, t4447: f64, t1573: f64, t961: f64, t1589: f64, t2940: f64, t1580: f64, t2904: f64, t952: f64, t959: f64, t4471: f64, t942: f64, t951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4480, t4482, t4483, t4485, t4487, t4488, t4489, t4491, t4493) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1015(t300, t4479, t4447, t1573, t961, t1589, t2940, t1580, t2904, t952, t959, t4471, t942, t951);
    (t4480, t4482, t4483, t4485, t4487, t4488, t4489, t4491, t4493)
}
