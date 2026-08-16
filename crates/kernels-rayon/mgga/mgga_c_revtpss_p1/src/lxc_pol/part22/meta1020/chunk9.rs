//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3547/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3547(t15731: f64, t4879: f64, t20020: f64, t3224: f64, t1025: f64, t127: f64, t19768: f64, t371: f64, t225: f64, t64686: f64, t366: f64, t1045: f64, t11656: f64, t11783: f64, t11927: f64, t15700: f64, t19693: f64, t19770: f64, t19861: f64, t3075: f64, t3117: f64, t3155: f64, t3162: f64, t3208: f64, t3211: f64, t42155: f64, t43044: f64, t43050: f64, t54672: f64, t6271: f64, t6278: f64, t65261: f64, t66062: f64) -> (f64, f64) {
    let t67473 = t4879 * t15731;
    let t67493 = t3224 * t20020;
    let t67499 = t1025 * t371 * t127 * t19768;
    let t67501 = t64686 * t225;
    let t67502 = t67501 * t366;
    let t67509 = 0.2540682555144873302e-2_f64 * t11656 * t19693 - 0.95275595817932748827e-4_f64 * t67473 + 0.85748036236139473944e-3_f64 * t11927 * t3117 * t6271 * t1045 * t3075 + 0.1270341277572436651e-2_f64 * t15700 * t54672 * t66062 - 0.57165357490759649296e-3_f64 * t42155 * t19861 + 0.85748036236139473944e-3_f64 * t43050 * t3117 * t65261 * t3155 - 0.42874018118069736972e-3_f64 * t43044 * t3117 * t65261 * t3162 - 0.28582678745379824648e-3_f64 * t67493 + 0.22866142996303859718e-2_f64 * t3211 * t19770 - 0.28582678745379824648e-3_f64 * t67499 + 0.42874018118069736972e-3_f64 * t67502 * t3208 - 0.21437009059034868486e-3_f64 * t11783 * t6278 - 0.42874018118069736972e-3_f64 * t3224 * t19770;
    (t67501, t67509)
}
