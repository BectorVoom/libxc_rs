//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1025/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1025<F: Float>(t4265: F, t8224: F, t1471: F, t220: F, t6298: F, t14475: F, t7706: F, t8212: F, t8216: F, t442: F, t8159: F, t1056: F, t1429: F, t1434: F, t14439: F, t1470: F, t21145: F, t2221: F, t2242: F, t26563: F, t26586: F, t26688: F, t26703: F, t26718: F, t3077: F, t4253: F, t476: F, t5937: F, t6247: F, t6267: F, t8192: F) -> (F,) {
    let t27308 = t4265 * t8224;
    let t27311 = t1471 * t6298 * t220;
    let t27315 = t1471 * t14475 * t7706;
    let t27319 = t4265 * t8212;
    let t27321 = t4265 * t8216;
    let t27331 = t8159 * t442;
    let t27333 = t1471 * t27331 * t1056;
    let t27344 = -0.9286875e-2 * t4253 * t26718 - 0.17687407407407407407e-1 * t27308 - 0.10612444444444444444e0 * t3077 * t27311 + 0.53062222222222222222e-1 * t1470 * t27315 + 0.58958024691358024691e-2 * t14439 - 0.29479012345679012345e-1 * t27319 - 0.35374814814814814815e-1 * t27321 + 0.24765e-1 * t6267 * t26563 - 0.619125e-2 * t476 * t26703 + 0.1857375e-1 * t6247 * t2221 + 0.1857375e-1 * t2242 * t5937 - 0.26531111111111111111e-1 * t1470 * t27333 - 0.1857375e-1 * t4253 * t26688 - 0.232171875e-2 * t21145 * t26586 + 0.9286875e-2 * t8192 * t1429 - 0.619125e-2 * t8192 * t1434;
    (t27344,)
}
