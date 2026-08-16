//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1267/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1267(t92794: f64, t93408: f64, t25624: f64, t3056: f64, t7143: f64, t25604: f64, t995: f64, t357: f64, t988: f64, t355: f64, t1096: f64, t11178: f64, t11207: f64, t11902: f64, t12040: f64, t12173: f64, t1976: f64, t1978: f64, t25461: f64, t25464: f64, t25483: f64, t25586: f64, t25597: f64, t25606: f64, t25634: f64, t25692: f64, t25699: f64, t25700: f64, t3075: f64, t3076: f64, t3270: f64, t3326: f64, t7102: f64, t7135: f64, t7140: f64, t7145: f64, t7151: f64, t7152: f64, t7153: f64, t7159: f64, t7160: f64, t999: f64) -> (f64, f64, f64) {
    let t93409 = t92794 + t93408;
    let t93429 = t25624 * t3056 * t7143;
    let t93436 = t995 * t25604;
    let t93437 = t357 * t988;
    let t93438 = t355 * t93437;
    let t93458 = 0.19756347548806534796e1_f64 * t7102 * t11207 + 0.26020884564615598386e1_f64 * t7151 * t7145 * t7135 * t3075 + 0.26020884564615598386e1_f64 * t7151 * t7145 * t25586 * t999 + 0.8673628188205199462e0_f64 * t7159 * t7160 * t1976 * t12173 - 0.52041769129231196772e1_f64 * t7151 * t7160 * t25483 * t1096 + 0.52041769129231196772e1_f64 * t93429 * t7153 - 0.19756347548806534796e1_f64 * t25692 * t3076 + 0.65854491829355115987e0_f64 * t11902 * t1978 + 0.10408353825846239354e2_f64 * t93436 * t25606 * t93438 + 0.15612530738769359031e2_f64 * t25699 * t7160 * t25700 * t1096 - 0.10408353825846239354e2_f64 * t25461 * t25597 + 0.39512695097613069591e1_f64 * t7140 * t11178 - 0.39512695097613069591e1_f64 * t7102 * t12040 - 0.19756347548806534796e1_f64 * t25634 * t3326 + 0.15612530738769359031e2_f64 * t7151 * t25464 * t7152 * t3270;
    (t93409, t93438, t93458)
}
