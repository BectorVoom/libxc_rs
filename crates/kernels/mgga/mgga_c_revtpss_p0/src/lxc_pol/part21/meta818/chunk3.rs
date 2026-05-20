//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3014/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3014<F: Float>(t3299: F, t53401: F, t11774: F, t16103: F, t53405: F, t16170: F, t372: F, t12116: F, t15688: F, t1042: F, t1045: F, t11145: F, t11666: F, t11706: F, t11852: F, t11994: F, t12003: F, t13312: F, t15691: F, t15696: F, t15811: F, t16226: F, t16229: F, t16230: F, t1651: F, t3059: F, t3127: F, t3204: F, t42328: F, t43069: F, t4839: F, t4872: F, t53545: F, t606: F, t905: F, t999: F) -> F {
    let t55100 = t3299 * t53401;
    let t55104 = t11774 * t53405 * t16103;
    let t55122 = t372 * t16170;
    let t55137 = t12116 * t15688;
    let t55140 = -F::cast_from(0.91464571985215438873e-2_f64) * t55100 * t16230 - F::cast_from(0.57165357490759649295e-3_f64) * t55104 + F::cast_from(0.43445671692977333464e-1_f64) * t3204 * t12003 * t4839 - F::cast_from(0.42874018118069736972e-3_f64) * t11994 * t15811 - F::cast_from(0.42874018118069736972e-3_f64) * t3127 * t1042 * t4872 * t13312 * t999 + F::cast_from(0.85748036236139473944e-3_f64) * t43069 * t15691 * t1045 * t3059 * t905 * t606 - F::cast_from(0.7145669686344956162e-3_f64) * t11774 * t55122 * t11706 + F::cast_from(0.42874018118069736972e-3_f64) * t42328 * t15696 * t11666 - F::cast_from(0.63517063878621832552e-3_f64) * t3127 * t1042 * t11852 * t1651 * t11145 + F::cast_from(0.17149607247227894789e-2_f64) * t16226 * t53545 * t16229 + F::cast_from(0.17149607247227894789e-2_f64) * t55137 * t16230;
    t55140
}
