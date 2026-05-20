//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3513/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3513<F: Float>(t1011: F, t15987: F, t18942: F, t15905: F, t55599: F, t6258: F, t905: F, t1045: F, t11675: F, t11875: F, t11927: F, t15599: F, t16022: F, t16070: F, t16089: F, t16096: F, t18946: F, t19639: F, t19641: F, t19645: F, t19741: F, t19836: F, t19947: F, t19951: F, t3091: F, t3092: F, t3117: F, t3241: F, t357: F, t42690: F, t42804: F, t42830: F, t4786: F, t53944: F, t6100: F, t6271: F, t63349: F, t64916: F) -> F {
    let t66423 = t1011 * t15987 * t18942;
    let t66431 = t55599 * t15905;
    let t66434 = t6258 * t905;
    let t66460 = F::new(35.0) / F::new(972.0) * t1011 * t53944 * t63349 + t3241 * t19947 / F::new(27.0) - F::new(2.0) / F::new(81.0) * t3241 * t19951 - t66423 / F::new(216.0) + F::cast_from(0.42874018118069736972e-3_f64) * t11927 * t3117 * t64916 * t1045 - F::cast_from(0.42874018118069736972e-3_f64) * t19741 * t16022 + F::cast_from(0.42874018118069736972e-3_f64) * t66431 * t16070 + F::cast_from(0.57165357490759649296e-3_f64) * t16089 * t3092 * t66434 * t16096 - F::cast_from(0.42874018118069736972e-3_f64) * t42690 * t3117 * t6271 * t42804 * t357 + F::cast_from(0.28582678745379824648e-3_f64) * t11675 * t19645 + F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t3092 * t18946 * t4786 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t3092 * t6100 * t15599 + F::cast_from(0.85748036236139473944e-3_f64) * t42830 * t19641 + F::cast_from(0.85748036236139473944e-3_f64) * t11875 * t3117 * t19836 * t19639;
    t66460
}
