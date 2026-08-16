//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3512/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3512(t15618: f64, t15984: f64, t1043: f64, t42622: f64, t19477: f64, t73: f64, t1011: f64, t15993: f64, t18913: f64, t18904: f64, t53972: f64, t11696: f64, t15601: f64, t15609: f64, t15615: f64, t16012: f64, t19450: f64, t19611: f64, t3091: f64, t3092: f64, t3095: f64, t3117: f64, t42621: f64, t43105: f64, t4788: f64, t4919: f64, t54126: f64, t54578: f64, t63344: f64, t63357: f64) -> (f64, f64, f64) {
    let t66376 = t15618 * t15984;
    let t66382 = t42622 * t1043;
    let t66395 = t19477 * t73;
    let t66403 = t1011 * t15993 * t18913;
    let t66406 = t1011 * t53972 * t18904;
    let t66414 = -t54126 / 243.0_f64 + 0.3811023832717309953e-3_f64 * t66376 + 0.14291339372689912324e-3_f64 * t3091 * t3092 * t19611 * t11696 - 0.25724410870841842184e-2_f64 * t42621 * t3117 * t19450 * t66382 + 0.25724410870841842183e-2_f64 * t43105 * t3117 * t19450 * t15609 + 0.57165357490759649296e-3_f64 * t54578 * t4788 + 0.57165357490759649296e-3_f64 * t15618 * t15615 + 0.28582678745379824648e-3_f64 * t3091 * t3092 * t66395 * t3095 + 0.28582678745379824648e-3_f64 * t15618 * t15601 + t66403 / 162.0_f64 + 7.0_f64 / 972.0_f64 * t66406 + t1011 * t4919 * t63357 / 108.0_f64 + 7.0_f64 / 648.0_f64 * t1011 * t16012 * t63344;
    (t66382, t66395, t66414)
}
