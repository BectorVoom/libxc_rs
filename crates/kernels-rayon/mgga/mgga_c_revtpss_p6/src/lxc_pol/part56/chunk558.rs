//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 558/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk558(t1015: f64, t4186: f64, t1012: f64, t3147: f64, t72: f64, t3088: f64, t3299: f64, t1668: f64, t3153: f64, t1043: f64, t3154: f64, t3117: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4886 = t1015 * t4186;
    let t4887 = t1012 * t4886;
    let t4890 = t3147 * t72;
    let t4891 = t3088 * t4890;
    let t4892 = t3299 * t4891;
    let t4893 = t1668 * t3153;
    let t4894 = t3154 * t1043;
    let t4895 = t4893 * t4894;
    let t4896 = t3117 * t4895;
    (t4887, t4890, t4891, t4892, t4893, t4896)
}
