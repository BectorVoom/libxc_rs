//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1347/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1347(t1907: f64, t6816: f64, t25082: f64, t8717: f64, t6941: f64, t7953: f64, t572: f64, t5883: f64, t7741: f64, t22633: f64, t7330: f64, t105823: f64, t5920: f64) -> (f64, f64, f64, f64, f64) {
    let t114820 = t6816 * t1907;
    let t114823 = 9.0_f64 * t25082 * t8717 * t114820;
    let t114838 = 9.0_f64 * t6941 * t7953;
    let t114841 = 18.0_f64 * t572 * t5883 * t7741;
    let t114844 = 6.0_f64 * t572 * t7330 * t22633;
    let t114847 = 18.0_f64 * t572 * t105823 * t5920;
    (t114823, t114838, t114841, t114844, t114847)
}
