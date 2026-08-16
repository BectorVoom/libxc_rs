//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3547/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3547<F: Float>(t15731: F, t4879: F, t20020: F, t3224: F, t1025: F, t127: F, t19768: F, t371: F, t225: F, t64686: F, t366: F, t1045: F, t11656: F, t11783: F, t11927: F, t15700: F, t19693: F, t19770: F, t19861: F, t3075: F, t3117: F, t3155: F, t3162: F, t3208: F, t3211: F, t42155: F, t43044: F, t43050: F, t54672: F, t6271: F, t6278: F, t65261: F, t66062: F) -> (F, F) {
    let t67473 = t4879 * t15731;
    let t67493 = t3224 * t20020;
    let t67499 = t1025 * t371 * t127 * t19768;
    let t67501 = t64686 * t225;
    let t67502 = t67501 * t366;
    let t67509 = F::cast_from(0.2540682555144873302e-2_f64) * t11656 * t19693 - F::cast_from(0.95275595817932748827e-4_f64) * t67473 + F::cast_from(0.85748036236139473944e-3_f64) * t11927 * t3117 * t6271 * t1045 * t3075 + F::cast_from(0.1270341277572436651e-2_f64) * t15700 * t54672 * t66062 - F::cast_from(0.57165357490759649296e-3_f64) * t42155 * t19861 + F::cast_from(0.85748036236139473944e-3_f64) * t43050 * t3117 * t65261 * t3155 - F::cast_from(0.42874018118069736972e-3_f64) * t43044 * t3117 * t65261 * t3162 - F::cast_from(0.28582678745379824648e-3_f64) * t67493 + F::cast_from(0.22866142996303859718e-2_f64) * t3211 * t19770 - F::cast_from(0.28582678745379824648e-3_f64) * t67499 + F::cast_from(0.42874018118069736972e-3_f64) * t67502 * t3208 - F::cast_from(0.21437009059034868486e-3_f64) * t11783 * t6278 - F::cast_from(0.42874018118069736972e-3_f64) * t3224 * t19770;
    (t67501, t67509)
}
