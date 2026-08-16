//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3546/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3546(t11922: f64, t20065: f64, t4892: f64, t4772: f64, t4866: f64, t15688: f64, t16584: f64, t1042: f64, t1045: f64, t11927: f64, t11933: f64, t13312: f64, t15618: f64, t15693: f64, t15696: f64, t15959: f64, t15969: f64, t19497: f64, t19620: f64, t20040: f64, t20105: f64, t3091: f64, t3092: f64, t3094: f64, t3115: f64, t3117: f64, t42155: f64, t43082: f64, t4781: f64, t4783: f64, t4806: f64, t4837: f64, t54578: f64, t55233: f64, t55247: f64, t66037: f64) -> (f64, f64) {
    let t67435 = t4892 * t11922 * t20065;
    let t67438 = t4772 * t4866;
    let t67458 = t16584 * t15688;
    let t67470 = -0.17149607247227894789e-2_f64 * t55233 + 0.57165357490759649296e-3_f64 * t67435 + 0.1270341277572436651e-3_f64 * t55247 - 0.85748036236139473944e-3_f64 * t3115 * t3117 * t67438 * t1045 + 0.22866142996303859718e-2_f64 * t11933 * t20105 + 0.85748036236139473944e-3_f64 * t11927 * t3117 * t19497 * t19620 + 0.28582678745379824648e-3_f64 * t3091 * t3092 * t4781 * t3094 * t13312 + 0.57165357490759649296e-3_f64 * t54578 * t4783 + 0.57165357490759649296e-3_f64 * t15618 * t15959 - 0.57165357490759649296e-3_f64 * t67458 * t15693 - 0.57165357490759649296e-3_f64 * t42155 * t20040 - 0.57165357490759649296e-3_f64 * t43082 * t15696 * t15969 + 0.47637797908966374414e-3_f64 * t4837 * t1042 * t4806 * t66037;
    (t67438, t67470)
}
