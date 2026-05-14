//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1161/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1161<F: Float>(t16926: F, t833: F, t3984: F, t12147: F, t5705: F, t1368: F, t12135: F, t12138: F, t12142: F, t12152: F, t16902: F, t16907: F, t16911: F, t16920: F, t16925: F, t3986: F, t5691: F) -> (F,) {
    let t16927 = t16926 * t833;
    let t16928 = t3984 * t16927;
    let t16933 = t12147 * t5705;
    let t16935 = t1368 * t16933 / 432.0;
    let t16936 = 7.0 / 648.0 * t1368 * t16902 - t1368 * t16907 / 54.0 - t1368 * t16911 / 288.0 - t12135 / 648.0 + t12138 / 864.0 + t12142 / 648.0 - t12152 / 432.0 + t1368 * t16920 / 144.0 - t16925 - t1368 * t16928 / 144.0 + t5691 * t3986 / 54.0 - t16935;
    (t16936,)
}
