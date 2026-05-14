//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 636/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk636<F: Float>(t499: F, t79: F, t5991: F, t6368: F, t4231: F, t6001: F, t4230: F, t470: F, t5967: F, t487: F, t1487: F, t2275: F, t4223: F, t492: F, t5885: F, t1506: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6369 = t79 * t499;
    let t6370 = t6369 * t5991;
    let t6371 = t6368 * t6370;
    let t6373 = t4231 * t6001;
    let t6374 = t4230 * t6373;
    let t6376 = t470 * t5967;
    let t6377 = t487 * t6376;
    let t6378 = t1487 * t6377;
    let t6380 = t4223 * t2275;
    let t6382 = t5885 * t492;
    let t6383 = t6382 * t1506;
    (t6369, t6370, t6371, t6373, t6374, t6377, t6378, t6380, t6382, t6383)
}
