//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1057/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1057<F: Float>(t1402: F, t3529: F, t5671: F, t1175: F, t1364: F, t2192: F, t5932: F, t1460: F, t458: F, t196: F, t19710: F, t14502: F, t1471: F, t2059: F, t1472: F, t14439: F, t14441: F, t14444: F, t14446: F, t1470: F, t19145: F, t19222: F, t19400: F, t2221: F, t3077: F, t4244: F, t4253: F, t460: F, t476: F, t5937: F, t5954: F, t6256: F, t6278: F) -> (F,) {
    let t21183 = t3529 * t1402;
    let t21184 = t21183 * t5671;
    let t21187 = t1175 * t1364;
    let t21188 = t2192 * t21187;
    let t21191 = t5932 * t21187;
    let t21196 = t1460 * t458;
    let t21203 = t19710 * t196;
    let t21207 = t1471 * t14502 * t2059;
    let t21210 = t1471 * t1472;
    let t21221 = -0.88437037037037037036e-1 * t6278 * t21184 + 0.371475e-1 * t4253 * t21188 - 0.9286875e-2 * t6256 * t21191 - 0.1857375e-1 * t4253 * t19400 + 0.24765e-1 * t21196 * t5954 + 0.11791604938271604938e-1 * t14439 - 0.35374814814814814814e-1 * t14441 - 0.17687407407407407407e-1 * t14444 - 0.29479012345679012345e-1 * t14446 + 0.619125e-2 * t21203 * t460 - 0.26531111111111111111e-1 * t1470 * t21207 - 0.53062222222222222222e-1 * t3077 * t21210 - 0.619125e-2 * t476 * t19222 + 0.9286875e-2 * t4244 * t2221 + 0.1857375e-1 * t1460 * t5937 + 0.9286875e-2 * t476 * t19145;
    (t21221,)
}
