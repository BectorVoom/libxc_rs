//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1109/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1109(t2013: f64, t9851: f64, t9847: f64, t2464: f64, t2465: f64, t7221: f64, t825: f64, t22672: f64, t2684: f64, t3295: f64, t10017: f64, t2615: f64) -> (f64, f64, f64, f64, f64) {
    let t28864 = 0.17041300423964777634e0_f64 * t2013 * t9851;
    let t28865 = t2013 * t9847;
    let t28873 = 0.17041300423964777634e0_f64 * t825 * t2464 * t2465 * t7221;
    let t28876 = 0.11928910296775344344e1_f64 * t2684 * t22672 * t3295;
    let t28878 = t2615 * t2464 * t10017;
    (t28864, t28865, t28873, t28876, t28878)
}
