//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1151/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1151(t13330: f64, t4278: f64, t3931: f64, t12510: f64, t14906: f64, t1113: f64, t9619: f64, t15846: f64, t1501: f64, t4246: f64, t3068: f64, t5261: f64, t9561: f64) -> (f64, f64, f64, f64, f64) {
    let t15881 = t4278 * t13330;
    let t15882 = t3931 * t15881;
    let t15885 = t12510 * t14906;
    let t15886 = t3931 * t15885;
    let t15889 = t9619 * t1113;
    let t15890 = t15846 * t15889;
    let t15891 = t3931 * t15890;
    let t15894 = t4246 * t1501;
    let t15895 = t3068 * t15894;
    let t15898 = t9561 * t5261;
    (t15882, t15886, t15891, t15895, t15898)
}
