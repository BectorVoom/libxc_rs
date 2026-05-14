//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 460/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk460<F: Float>(t201: F, t6070: F, t1856: F, t457: F, t1451: F, t228: F, t1859: F, t5542: F, t615: F, t1173: F, t1864: F, t495: F, t1525: F, t1907: F, t461: F, t1184: F, t1867: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6071 = t6070 * t201;
    let t6073 = t1856 * t457;
    let t6077 = t228 * t1451;
    let t6080 = t1859 * t457;
    let t6086 = t5542 * t615;
    let t6093 = t1173 * t1864;
    let t6096 = t1864 * t495;
    let t6099 = t615 * t1525;
    let t6102 = t461 * t1907;
    let t6105 = t1907 * t495;
    let t6108 = t1867 * t1184;
    (t6071, t6073, t6077, t6080, t6086, t6093, t6096, t6099, t6102, t6105, t6108)
}
