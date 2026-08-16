//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1377;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta340(t225: f64, t4143: f64, t4145: f64, t1496: f64, t9541: f64, t2427: f64, t4101: f64, t2528: f64, t4199: f64, t2663: f64, t4211: f64, t2535: f64, t1471: f64, t32: f64, t4095: f64, t67: f64, t758: f64, t118: f64, t1474: f64, t2375: f64, t4094: f64, t706: f64, t4162: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13053, t13065, t13087, t13105, t13107, t13109, t13113) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1377(t225, t4143, t4145, t1496, t9541, t2427, t4101, t2528, t4199, t2663, t4211, t2535);
        let (t13115, t13121, t13124, t13133, t13176) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1378(t1471, t32, t4095, t67, t758, t118, t1474, t2375, t4094, t706, t4162, t68);
    (t13053, t13065, t13087, t13105, t13107, t13109, t13113, t13115, t13121, t13124, t13133, t13176)
}
