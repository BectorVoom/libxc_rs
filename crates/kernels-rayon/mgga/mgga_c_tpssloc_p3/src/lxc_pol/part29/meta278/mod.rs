//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1285;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1286;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1287;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta278(t462: f64, t8010: f64, t1760: f64, t7301: f64, t7300: f64, t1720: f64, t2144: f64, t131: f64, t7998: f64, t2130: f64, t1932: f64, rho1: f64, t2133: f64, t2132: f64, t7573: f64, t1714: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8011, t8014, t8015, t8018, t8020) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1285(t462, t8010, t1760, t7301, t7300, t1720, t2144, t131, t7998);
        let (t8026, t8027) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1286(t2130, t1932, rho1);
        let (t8028, t8031, t8034) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1287(t2133, t8027, t2132, t7573, t1714, t460);
    (t8011, t8014, t8015, t8018, t8020, t8026, t8027, t8028, t8031, t8034)
}
