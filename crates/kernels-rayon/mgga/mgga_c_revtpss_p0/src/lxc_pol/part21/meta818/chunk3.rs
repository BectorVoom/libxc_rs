//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3014/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3014(t3299: f64, t53401: f64, t11774: f64, t16103: f64, t53405: f64, t16170: f64, t372: f64, t12116: f64, t15688: f64, t1042: f64, t1045: f64, t11145: f64, t11666: f64, t11706: f64, t11852: f64, t11994: f64, t12003: f64, t13312: f64, t15691: f64, t15696: f64, t15811: f64, t16226: f64, t16229: f64, t16230: f64, t1651: f64, t3059: f64, t3127: f64, t3204: f64, t42328: f64, t43069: f64, t4839: f64, t4872: f64, t53545: f64, t606: f64, t905: f64, t999: f64) -> f64 {
    let t55100 = t3299 * t53401;
    let t55104 = t11774 * t53405 * t16103;
    let t55122 = t372 * t16170;
    let t55137 = t12116 * t15688;
    let t55140 = -0.91464571985215438873e-2_f64 * t55100 * t16230 - 0.57165357490759649295e-3_f64 * t55104 + 0.43445671692977333464e-1_f64 * t3204 * t12003 * t4839 - 0.42874018118069736972e-3_f64 * t11994 * t15811 - 0.42874018118069736972e-3_f64 * t3127 * t1042 * t4872 * t13312 * t999 + 0.85748036236139473944e-3_f64 * t43069 * t15691 * t1045 * t3059 * t905 * t606 - 0.7145669686344956162e-3_f64 * t11774 * t55122 * t11706 + 0.42874018118069736972e-3_f64 * t42328 * t15696 * t11666 - 0.63517063878621832552e-3_f64 * t3127 * t1042 * t11852 * t1651 * t11145 + 0.17149607247227894789e-2_f64 * t16226 * t53545 * t16229 + 0.17149607247227894789e-2_f64 * t55137 * t16230;
    t55140
}
