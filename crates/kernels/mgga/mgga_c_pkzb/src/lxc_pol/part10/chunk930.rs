//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 930/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk930<F: Float>(t7226: F, t7227: F, t730: F, t2816: F, t702: F, t1096: F, t1932: F, t1917: F, t2819: F, t1940: F, t2815: F, t1095: F, t5873: F, t2849: F, t721: F, t1108: F, t1971: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7228 = t7226 * t7227;
    let t7230 = 0.10254018858216406658e4 * t730 * t7228;
    let t7231 = t2816 * t702;
    let t7234 = t1096 * t1932;
    let t7237 = t2819 * t1917;
    let t7240 = t2815 * t1940;
    let t7241 = t7240 * t702;
    let t7244 = t2819 * t1932;
    let t7247 = t1095 * t5873;
    let t7248 = t7247 * t1917;
    let t7255 = t2849 * t721;
    let t7258 = t1108 * t1971;
    (t7228, t7230, t7231, t7234, t7237, t7241, t7244, t7248, t7255, t7258)
}
