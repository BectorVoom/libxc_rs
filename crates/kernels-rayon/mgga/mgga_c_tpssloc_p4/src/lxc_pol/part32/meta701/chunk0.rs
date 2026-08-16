//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2197/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2197(t2020: f64, t97804: f64, t15868: f64, t1983: f64, t7753: f64, t22574: f64, t74032: f64, t8643: f64, t28237: f64, t532: f64, t6879: f64, t510: f64, t652: f64, t96729: f64) -> (f64, f64, f64, f64, f64) {
    let t97805 = t97804 * t2020;
    let t97808 = 2.0_f64 * t1983 * t7753 * t15868;
    let t97811 = 3.0_f64 * t22574 * t8643 * t74032;
    let t97817 = t532 * t28237;
    let t97820 = 3.0_f64 * t1983 * t97817 * t6879;
    let t97829 = 2.0_f64 * t652 * t510 * t96729;
    (t97805, t97808, t97811, t97820, t97829)
}
