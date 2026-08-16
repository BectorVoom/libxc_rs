//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3535/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3535(t11947: f64, t20016: f64, t19620: f64, t66061: f64, t1045: f64, t11696: f64, t11703: f64, t11705: f64, t11774: f64, t15618: f64, t15691: f64, t15700: f64, t1592: f64, t15965: f64, t16089: f64, t16222: f64, t16226: f64, t19501: f64, t19611: f64, t19981: f64, t19997: f64, t20099: f64, t3059: f64, t3091: f64, t3092: f64, t3181: f64, t372: f64, t42360: f64, t43069: f64, t43151: f64, t4866: f64, t4899: f64, t53545: f64, t54943: f64, t6339: f64, t65876: f64) -> (f64, f64) {
    let t67072 = t11947 * t20016;
    let t67090 = t66061 * t19620;
    let t67102 = 0.23818898954483187207e-3_f64 * t3091 * t11703 * t19611 * t11705 + 0.19055119163586549765e-2_f64 * t16089 * t11703 * t20099 * t65876 - 0.57165357490759649296e-3_f64 * t15618 * t15965 + 0.14481890564325777821e-1_f64 * t43151 * t6339 - 0.30488190661738479624e-2_f64 * t67072 + 0.42874018118069736972e-3_f64 * t42360 * t6339 - 0.14291339372689912324e-3_f64 * t4899 * t3092 * t19501 * t11696 - 0.23818898954483187207e-3_f64 * t4899 * t11703 * t19501 * t11705 - 0.1270341277572436651e-3_f64 * t54943 + 0.57165357490759649296e-3_f64 * t43069 * t15691 * t1045 * t1592 * t3059 - 0.47637797908966374414e-3_f64 * t11774 * t16222 * t67090 + 0.11433071498151929859e-2_f64 * t16226 * t53545 * t19997 + 0.95275595817932748826e-3_f64 * t15700 * t372 * t3181 * t4866 * t19981;
    (t67090, t67102)
}
