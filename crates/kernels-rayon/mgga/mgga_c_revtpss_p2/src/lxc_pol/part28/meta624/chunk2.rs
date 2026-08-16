//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2216/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2216(t15734: f64, t25522: f64, t15816: f64, t7121: f64, t15822: f64, t25504: f64, t15794: f64, t25580: f64, t1047: f64, t15959: f64, t16104: f64, t25517: f64, t27450: f64, t3136: f64, t3157: f64, t4783: f64, t4825: f64, t93646: f64, t93673: f64, t93683: f64, t93685: f64, t93752: f64, t93821: f64) -> f64 {
    let t100166 = t25522 * t15734;
    let t100168 = t15816 * t7121;
    let t100173 = t15822 * t25504;
    let t100186 = 0.57165357490759649296e-3_f64 * t25580 * t15794;
    let t100187 = 0.30488190661738479624e-2_f64 * t93646 * t4825 - 0.3811023832717309953e-3_f64 * t100166 + 0.85748036236139473944e-3_f64 * t100168 * t1047 + 0.42874018118069736972e-3_f64 * t27450 * t3136 + 0.85748036236139473944e-3_f64 * t100173 * t3157 + 0.57165357490759649296e-3_f64 * t93821 * t4783 + 0.57165357490759649296e-3_f64 * t25517 * t15959 - 0.20325460441158986416e-2_f64 * t93673 - 0.57165357490759649296e-3_f64 * t93752 * t16104 - 0.57165357490759649296e-3_f64 * t93683 - 0.28582678745379824648e-3_f64 * t93685 - t100186;
    t100187
}
