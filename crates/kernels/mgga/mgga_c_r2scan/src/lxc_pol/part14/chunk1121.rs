//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1121/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1121<F: Float>(t1039: F, t11470: F, t12227: F, t42294: F, t42302: F, t42304: F, t42307: F, t42310: F, t42313: F, t42316: F, t42320: F, t42326: F, t42330: F, t42334: F, t42339: F, t42344: F, t42346: F, t42349: F, t860: F) -> (F,) {
    let t42364 = t1039 * t11470 + 2.0 * t12227 * t860 - t42294 - t42302 + t42304 - t42307 - t42310 - t42313 - t42316 + t42320 - t42326 + t42330 - t42334 + t42339 - t42344 + t42346 - t42349;
    (t42364,)
}
