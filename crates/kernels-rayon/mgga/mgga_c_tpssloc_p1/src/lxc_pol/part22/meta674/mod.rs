//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2231;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta674(t13822: f64, t17777: f64, t973: f64, t2986: f64, t4514: f64, t48019: f64, t48046: f64, t10236: f64, t17691: f64, t13779: f64, t17183: f64, t16558: f64, t2989: f64, t10224: f64, t5828: f64, t42875: f64, t5817: f64, t17763: f64, t2960: f64, t18057: f64, t225: f64, t18059: f64, t1020: f64, t17960: f64, t248: f64, t3101: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61472, t61489, t61495, t61528, t61557, t61589) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2231(t13822, t17777, t973, t2986, t4514, t48019, t48046, t10236, t17691, t13779, t17183, t16558, t2989);
        let (t61597, t61600, t61602, t61621, t61646, t61655) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2232(t10224, t5828, t973, t42875, t5817, t17763, t2960, t18057, t225, t18059, t1020, t17960, t248, t3101);
    (t61472, t61489, t61495, t61528, t61557, t61589, t61597, t61600, t61602, t61621, t61646, t61655)
}
