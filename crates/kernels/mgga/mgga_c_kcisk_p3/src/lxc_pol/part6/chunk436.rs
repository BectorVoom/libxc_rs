//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 436/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk436<F: Float>(t1035: F, t3245: F, t207: F, t3174: F, t206: F, t2689: F, t190: F, t3127: F, t214: F, t1045: F, t3132: F, t1042: F, t1050: F) -> (F, F, F, F, F, F, F, F) {
    let t3246 = t1035 * t3245;
    let t3248 = t207 * t3174;
    let t3249 = t1035 * t3248;
    let t3251 = t206 * t2689;
    let t3253 = t3127 * t190;
    let t3254 = t3253 * t214;
    let t3256 = t3132 * t1045;
    let t3258 = t1042 * t1050;
    (t3246, t3248, t3249, t3251, t3253, t3254, t3256, t3258)
}
