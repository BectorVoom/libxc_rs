//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1818/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1818(t22716: f64, t7701: f64, t1834: f64, t212: f64, t22642: f64, t6890: f64, t81267: f64, t26215: f64, t81228: f64, t81326: f64, t6897: f64, t6907: f64, t90544: f64) -> (f64, f64, f64, f64, f64) {
    let t90659 = t22716 * t7701;
    let t90663 = t22642 * t212 * t1834 * t6890;
    let t90670 = 0.16449340668482264365e-1_f64 * t81267;
    let t90686 = t81228 * t81326 * t26215;
    let t90701 = t6897 * t90544 * t6907;
    (t90659, t90663, t90670, t90686, t90701)
}
