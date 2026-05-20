//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3512/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3512<F: Float>(t15618: F, t15984: F, t1043: F, t42622: F, t19477: F, t73: F, t1011: F, t15993: F, t18913: F, t18904: F, t53972: F, t11696: F, t15601: F, t15609: F, t15615: F, t16012: F, t19450: F, t19611: F, t3091: F, t3092: F, t3095: F, t3117: F, t42621: F, t43105: F, t4788: F, t4919: F, t54126: F, t54578: F, t63344: F, t63357: F) -> (F, F, F) {
    let t66376 = t15618 * t15984;
    let t66382 = t42622 * t1043;
    let t66395 = t19477 * t73;
    let t66403 = t1011 * t15993 * t18913;
    let t66406 = t1011 * t53972 * t18904;
    let t66414 = -t54126 / F::new(243.0) + F::cast_from(0.3811023832717309953e-3_f64) * t66376 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t3092 * t19611 * t11696 - F::cast_from(0.25724410870841842184e-2_f64) * t42621 * t3117 * t19450 * t66382 + F::cast_from(0.25724410870841842183e-2_f64) * t43105 * t3117 * t19450 * t15609 + F::cast_from(0.57165357490759649296e-3_f64) * t54578 * t4788 + F::cast_from(0.57165357490759649296e-3_f64) * t15618 * t15615 + F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t3092 * t66395 * t3095 + F::cast_from(0.28582678745379824648e-3_f64) * t15618 * t15601 + t66403 / F::new(162.0) + F::new(7.0) / F::new(972.0) * t66406 + t1011 * t4919 * t63357 / F::new(108.0) + F::new(7.0) / F::new(648.0) * t1011 * t16012 * t63344;
    (t66382, t66395, t66414)
}
