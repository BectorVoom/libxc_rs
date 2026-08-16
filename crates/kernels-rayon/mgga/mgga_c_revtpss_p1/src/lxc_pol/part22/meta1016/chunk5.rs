//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3513/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3513(t1011: f64, t15987: f64, t18942: f64, t15905: f64, t55599: f64, t6258: f64, t905: f64, t1045: f64, t11675: f64, t11875: f64, t11927: f64, t15599: f64, t16022: f64, t16070: f64, t16089: f64, t16096: f64, t18946: f64, t19639: f64, t19641: f64, t19645: f64, t19741: f64, t19836: f64, t19947: f64, t19951: f64, t3091: f64, t3092: f64, t3117: f64, t3241: f64, t357: f64, t42690: f64, t42804: f64, t42830: f64, t4786: f64, t53944: f64, t6100: f64, t6271: f64, t63349: f64, t64916: f64) -> f64 {
    let t66423 = t1011 * t15987 * t18942;
    let t66431 = t55599 * t15905;
    let t66434 = t6258 * t905;
    let t66460 = 35.0_f64 / 972.0_f64 * t1011 * t53944 * t63349 + t3241 * t19947 / 27.0_f64 - 2.0_f64 / 81.0_f64 * t3241 * t19951 - t66423 / 216.0_f64 + 0.42874018118069736972e-3_f64 * t11927 * t3117 * t64916 * t1045 - 0.42874018118069736972e-3_f64 * t19741 * t16022 + 0.42874018118069736972e-3_f64 * t66431 * t16070 + 0.57165357490759649296e-3_f64 * t16089 * t3092 * t66434 * t16096 - 0.42874018118069736972e-3_f64 * t42690 * t3117 * t6271 * t42804 * t357 + 0.28582678745379824648e-3_f64 * t11675 * t19645 + 0.28582678745379824648e-3_f64 * t3091 * t3092 * t18946 * t4786 + 0.14291339372689912324e-3_f64 * t3091 * t3092 * t6100 * t15599 + 0.85748036236139473944e-3_f64 * t42830 * t19641 + 0.85748036236139473944e-3_f64 * t11875 * t3117 * t19836 * t19639;
    t66460
}
