//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1421/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1421(t15993: f64, t4574: f64, t1011: f64, t1012: f64, t11821: f64, t11922: f64, t4906: f64, t3115: f64, t4895: f64, t4892: f64, t140: f64, t4886: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15994 = t15993 * t4574;
    let t15996 = t1011 * t15994 / 324.0_f64;
    let t16012 = t1012 * t11821;
    let t16035 = t11922 * t4906;
    let t16037 = 0.28582678745379824648e-3_f64 * t3115 * t16035;
    let t16055 = t11922 * t4895;
    let t16057 = 0.57165357490759649296e-3_f64 * t4892 * t16055;
    let t16060 = t140 * t4886;
    (t15996, t16012, t16035, t16037, t16055, t16057, t16060)
}
