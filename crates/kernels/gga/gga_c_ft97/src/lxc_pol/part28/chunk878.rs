//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 878/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk878<F: Float>(t1286: F, t34357: F, t376: F, t108: F, t34482: F, t34576: F, t136044: F, t136058: F, t1564: F, t25990: F, t25996: F, t28: F, t32019: F, t32396: F, t3266: F, t3289: F, t34589: F, t379: F, t38921: F, t5495: F, t5501: F, t5502: F, t5507: F, t5748: F, t6414: F, t7166: F, t8411: F, t942: F) -> (F,) {
    let t144381 = t1286 * t376 * t34357;
    let t144393 = t34482 * t108;
    let t144405 = t1286 * t376 * t34576;
    let t144411 = -t136044 / 9.0 - 2.0 / 3.0 * t1286 * t28 * t5507 * t5748 * t942 + t144381 / 9.0 - t136058 + t6414 * t32396 / 3.0 - 4.0 * t5501 * t38921 * t32019 * t3266 + 2.0 * t5501 * t8411 * t5502 * t25996 - t5501 * t1564 * t144393 * t379 / 18.0 + 2.0 * t5501 * t8411 * t5502 * t25990 + t5495 * t34589 / 3.0 - t144405 / 18.0 - t1286 * t28 * t7166 * t3289 / 3.0;
    (t144411,)
}
