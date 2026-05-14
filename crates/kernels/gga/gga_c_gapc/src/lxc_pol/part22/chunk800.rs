//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 800/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk800<F: Float>(t1096: F, t2822: F, t2470: F, t1066: F, t2468: F, t2902: F, t761: F, t3221: F, t1474: F, t277: F, t1051: F, t2043: F, t6808: F, t3244: F, t291: F, t467: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10088 = t1096 * t2822;
    let t10091 = t1096 * t2470;
    let t10099 = t1066 * t2468;
    let t10102 = t2902 * t761;
    let t10103 = t10102 * t3221;
    let t10105 = t1474 * t277;
    let t10106 = t10105 * t3221;
    let t10108 = t2043 * t1051;
    let t10110 = t2902 * t6808;
    let t10111 = t10110 * t3244;
    let t10113 = t467 * t291;
    (t10088, t10091, t10099, t10102, t10103, t10105, t10106, t10108, t10110, t10111, t10113)
}
