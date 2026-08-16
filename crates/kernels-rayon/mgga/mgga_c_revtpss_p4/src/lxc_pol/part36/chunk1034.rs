//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1034/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1034(t23652: f64, t23819: f64, t1045: f64, t373: f64, t1042: f64, t11632: f64, t23641: f64, t11250: f64, t1668: f64, t6244: f64, t3117: f64, t1469: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23820 = t23652 + t23819;
    let t23822 = t373 * t23820 * t1045;
    let t23823 = t1042 * t23822;
    let t23829 = t23641 * t11632;
    let t23830 = t1042 * t23829;
    let t23833 = t23641 * t11250;
    let t23834 = t1042 * t23833;
    let t23837 = t6244 * t1668;
    let t23838 = t23837 * t1045;
    let t23839 = t3117 * t23838;
    let t23842 = t5825 * t1469;
    (t23820, t23823, t23830, t23834, t23837, t23839, t23842)
}
