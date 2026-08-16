//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2239;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta678(t17659: f64, t3117: f64, t1041: f64, t17187: f64, t248: f64, t3051: f64, t10422: f64, t17704: f64, t3070: f64, t17680: f64, t13969: f64, t17692: f64, t14077: f64, t4630: f64, t10482: f64, t5872: f64, t10413: f64, t17924: f64, t17959: f64, t376: f64, t10480: f64, t17672: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61977, t61981, t62013, t62032, t62038) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2239(t17659, t3117, t1041, t17187, t248, t3051, t10422, t17704, t3070, t17680, t13969, t17692);
        let (t62049, t62079, t62085, t62091, t62099) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2240(t14077, t4630, t10482, t5872, t10413, t10422, t17924, t17959, t376, t10480, t13969, t17672);
    (t61977, t61981, t62013, t62032, t62038, t62049, t62079, t62085, t62091, t62099)
}
