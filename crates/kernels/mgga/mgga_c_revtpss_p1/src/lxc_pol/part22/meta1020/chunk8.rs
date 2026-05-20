//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3546/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3546<F: Float>(t11922: F, t20065: F, t4892: F, t4772: F, t4866: F, t15688: F, t16584: F, t1042: F, t1045: F, t11927: F, t11933: F, t13312: F, t15618: F, t15693: F, t15696: F, t15959: F, t15969: F, t19497: F, t19620: F, t20040: F, t20105: F, t3091: F, t3092: F, t3094: F, t3115: F, t3117: F, t42155: F, t43082: F, t4781: F, t4783: F, t4806: F, t4837: F, t54578: F, t55233: F, t55247: F, t66037: F) -> (F, F) {
    let t67435 = t4892 * t11922 * t20065;
    let t67438 = t4772 * t4866;
    let t67458 = t16584 * t15688;
    let t67470 = -F::cast_from(0.17149607247227894789e-2_f64) * t55233 + F::cast_from(0.57165357490759649296e-3_f64) * t67435 + F::cast_from(0.1270341277572436651e-3_f64) * t55247 - F::cast_from(0.85748036236139473944e-3_f64) * t3115 * t3117 * t67438 * t1045 + F::cast_from(0.22866142996303859718e-2_f64) * t11933 * t20105 + F::cast_from(0.85748036236139473944e-3_f64) * t11927 * t3117 * t19497 * t19620 + F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t3092 * t4781 * t3094 * t13312 + F::cast_from(0.57165357490759649296e-3_f64) * t54578 * t4783 + F::cast_from(0.57165357490759649296e-3_f64) * t15618 * t15959 - F::cast_from(0.57165357490759649296e-3_f64) * t67458 * t15693 - F::cast_from(0.57165357490759649296e-3_f64) * t42155 * t20040 - F::cast_from(0.57165357490759649296e-3_f64) * t43082 * t15696 * t15969 + F::cast_from(0.47637797908966374414e-3_f64) * t4837 * t1042 * t4806 * t66037;
    (t67438, t67470)
}
