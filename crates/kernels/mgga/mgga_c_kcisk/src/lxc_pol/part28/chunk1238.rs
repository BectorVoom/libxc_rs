//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1238/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1238<F: Float>(t8820: F, t9670: F, t7261: F, t2508: F, t9956: F, t415: F, t2789: F, t8665: F, t8514: F, t9665: F, t1775: F, t8940: F, t9687: F, t20: F, t2447: F, t2454: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t35211 = t9670 * t8820;
    let t35212 = t7261 * t35211;
    let t35221 = t2508 * t9956;
    let t35222 = t415 * t35221;
    let t35224 = t8665 * t2789;
    let t35225 = t415 * t35224;
    let t35229 = t9665 * t8514;
    let t35230 = t1775 * t35229;
    let t35233 = t9687 * t8940;
    let t35234 = t415 * t35233;
    let t35237 = t2447 * t2454 * t20;
    (t35211, t35212, t35221, t35222, t35224, t35225, t35229, t35230, t35233, t35234, t35237)
}
