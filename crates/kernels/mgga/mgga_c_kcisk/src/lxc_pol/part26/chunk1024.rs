//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1024/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1024<F: Float>(t2242: F, t458: F, t1173: F, t19352: F, t5932: F, t1364: F, t2191: F, t19434: F, t25554: F, t416: F, t196: F, t25906: F, t140: F, t21154: F, t21163: F, t21168: F, t21169: F, t21177: F, t21180: F, t26591: F, t26595: F, t26684: F, t26722: F, t4253: F, t460: F, t479: F, t5928: F, t5954: F, t6256: F, t6267: F) -> (F,) {
    let t27277 = t2242 * t458;
    let t27281 = t2242 * t1173;
    let t27284 = t5932 * t19352;
    let t27287 = t2191 * t1364;
    let t27288 = t19434 * t27287;
    let t27291 = t416 * t25554;
    let t27298 = t25906 * t196;
    let t27304 = -0.46434375e-2 * t6256 * t26591 + 0.9286875e-2 * t6256 * t26595 + 0.24765e-1 * t27277 * t5954 - 0.35374814814814814815e-1 * t21154 + 0.9286875e-2 * t27281 * t5928 - 0.9286875e-2 * t6256 * t27284 + 0.371475e-1 * t4253 * t27288 - t21163 - 0.39796666666666666666e-1 * t140 * t479 * t27291 + 0.123825e-1 * t6267 * t26722 - t21168 - 0.70749629629629629628e-1 * t21169 + 0.619125e-2 * t27298 * t460 + 0.70749629629629629628e-1 * t21177 - t21180 - 0.1857375e-1 * t4253 * t26684;
    (t27304,)
}
