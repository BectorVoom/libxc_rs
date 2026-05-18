//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1267/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1267<F: Float>(t92794: F, t93408: F, t25624: F, t3056: F, t7143: F, t25604: F, t995: F, t357: F, t988: F, t355: F, t1096: F, t11178: F, t11207: F, t11902: F, t12040: F, t12173: F, t1976: F, t1978: F, t25461: F, t25464: F, t25483: F, t25586: F, t25597: F, t25606: F, t25634: F, t25692: F, t25699: F, t25700: F, t3075: F, t3076: F, t3270: F, t3326: F, t7102: F, t7135: F, t7140: F, t7145: F, t7151: F, t7152: F, t7153: F, t7159: F, t7160: F, t999: F) -> (F, F, F) {
    let t93409 = t92794 + t93408;
    let t93429 = t25624 * t3056 * t7143;
    let t93436 = t995 * t25604;
    let t93437 = t357 * t988;
    let t93438 = t355 * t93437;
    let t93458 = F::new(0.19756347548806534796e1) * t7102 * t11207 + F::new(0.26020884564615598386e1) * t7151 * t7145 * t7135 * t3075 + F::new(0.26020884564615598386e1) * t7151 * t7145 * t25586 * t999 + F::new(0.8673628188205199462e0) * t7159 * t7160 * t1976 * t12173 - F::new(0.52041769129231196772e1) * t7151 * t7160 * t25483 * t1096 + F::new(0.52041769129231196772e1) * t93429 * t7153 - F::new(0.19756347548806534796e1) * t25692 * t3076 + F::new(0.65854491829355115987e0) * t11902 * t1978 + F::new(0.10408353825846239354e2) * t93436 * t25606 * t93438 + F::new(0.15612530738769359031e2) * t25699 * t7160 * t25700 * t1096 - F::new(0.10408353825846239354e2) * t25461 * t25597 + F::new(0.39512695097613069591e1) * t7140 * t11178 - F::new(0.39512695097613069591e1) * t7102 * t12040 - F::new(0.19756347548806534796e1) * t25634 * t3326 + F::new(0.15612530738769359031e2) * t7151 * t25464 * t7152 * t3270;
    (t93409, t93438, t93458)
}
