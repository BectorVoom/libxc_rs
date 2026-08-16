//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2240/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2240(t14077: f64, t4630: f64, t10482: f64, t5872: f64, t10413: f64, t10422: f64, t17924: f64, t17959: f64, t376: f64, t10480: f64, t13969: f64, t17672: f64) -> (f64, f64, f64, f64, f64) {
    let t62049 = t14077 * t4630;
    let t62079 = t5872 * t10482;
    let t62085 = t10413 * t10422 * t17924;
    let t62091 = t376 * t17959;
    let t62099 = t10480 * t13969 * t17672;
    (t62049, t62079, t62085, t62091, t62099)
}
