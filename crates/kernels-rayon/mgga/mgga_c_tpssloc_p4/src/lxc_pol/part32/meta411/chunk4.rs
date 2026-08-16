//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1587/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1587(t18215: f64, t4900: f64, t11570: f64, t5392: f64, t11569: f64, t1180: f64, t15284: f64, t15287: f64, t15300: f64, t15307: f64, t18321: f64, t18443: f64, t18447: f64, t18452: f64, t18455: f64, t18458: f64, t18460: f64, t3447: f64, t4889: f64, t4937: f64) -> f64 {
    let t18466 = t4900 * t18215;
    let t18469 = t11570 * t5392;
    let t18470 = t11569 * t18469;
    let t18473 = -t15284 - t15287 - 0.86419753086419753084e-3_f64 * t3447 * t18443 + 0.18518518518518518518e-3_f64 * t18447 + 0.44444444444444444444e-2_f64 * t4889 * t4937 - 0.18518518518518518518e-3_f64 * t18452 - 0.9259259259259259259e-4_f64 * t18455 + 0.12345679012345679012e-3_f64 * t18458 + 0.49382716049382716047e-3_f64 * t18460 - 0.27160493827160493827e-2_f64 * t18321 * t1180 + 0.12345679012345679012e-3_f64 * t15300 + 0.49382716049382716047e-3_f64 * t15307 + 0.74074074074074074072e-3_f64 * t3447 * t18466 - 0.37037037037037037036e-3_f64 * t3447 * t18470;
    t18473
}
