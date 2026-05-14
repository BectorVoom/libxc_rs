//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1053/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1053<F: Float>(t20940: F, t6322: F, t4230: F, t1493: F, t6310: F, t19873: F, t6369: F, t6368: F, t1471: F, t220: F, t4277: F, t19123: F, t6279: F, t12825: F, t451: F, t19109: F) -> (F, F, F, F, F, F, F, F) {
    let t21093 = t6322 * t20940;
    let t21094 = t4230 * t21093;
    let t21096 = t6310 * t1493;
    let t21098 = t6369 * t19873;
    let t21099 = t6368 * t21098;
    let t21104 = t1471 * t4277 * t220;
    let t21110 = t6279 * t19123;
    let t21113 = t12825 * t451;
    let t21114 = t21113 * t19109;
    (t21093, t21094, t21096, t21098, t21099, t21104, t21110, t21114)
}
