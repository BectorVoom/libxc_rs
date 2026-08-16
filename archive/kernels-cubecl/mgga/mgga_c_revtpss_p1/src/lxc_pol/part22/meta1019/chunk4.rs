//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3535/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3535<F: Float>(t11947: F, t20016: F, t19620: F, t66061: F, t1045: F, t11696: F, t11703: F, t11705: F, t11774: F, t15618: F, t15691: F, t15700: F, t1592: F, t15965: F, t16089: F, t16222: F, t16226: F, t19501: F, t19611: F, t19981: F, t19997: F, t20099: F, t3059: F, t3091: F, t3092: F, t3181: F, t372: F, t42360: F, t43069: F, t43151: F, t4866: F, t4899: F, t53545: F, t54943: F, t6339: F, t65876: F) -> (F, F) {
    let t67072 = t11947 * t20016;
    let t67090 = t66061 * t19620;
    let t67102 = F::cast_from(0.23818898954483187207e-3_f64) * t3091 * t11703 * t19611 * t11705 + F::cast_from(0.19055119163586549765e-2_f64) * t16089 * t11703 * t20099 * t65876 - F::cast_from(0.57165357490759649296e-3_f64) * t15618 * t15965 + F::cast_from(0.14481890564325777821e-1_f64) * t43151 * t6339 - F::cast_from(0.30488190661738479624e-2_f64) * t67072 + F::cast_from(0.42874018118069736972e-3_f64) * t42360 * t6339 - F::cast_from(0.14291339372689912324e-3_f64) * t4899 * t3092 * t19501 * t11696 - F::cast_from(0.23818898954483187207e-3_f64) * t4899 * t11703 * t19501 * t11705 - F::cast_from(0.1270341277572436651e-3_f64) * t54943 + F::cast_from(0.57165357490759649296e-3_f64) * t43069 * t15691 * t1045 * t1592 * t3059 - F::cast_from(0.47637797908966374414e-3_f64) * t11774 * t16222 * t67090 + F::cast_from(0.11433071498151929859e-2_f64) * t16226 * t53545 * t19997 + F::cast_from(0.95275595817932748826e-3_f64) * t15700 * t372 * t3181 * t4866 * t19981;
    (t67090, t67102)
}
