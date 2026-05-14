//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 843/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk843<F: Float>(t12061: F, t29636: F, t12002: F, t15989: F, t22564: F, t22575: F, t22583: F, t28371: F, t28375: F, t28379: F, t28383: F, t28387: F, t28391: F, t12018: F, t12059: F, t18546: F, t1966: F, t24785: F, t2605: F, t28352: F, t28354: F, t28356: F, t28360: F, t28441: F, t28444: F, t28461: F, t28464: F, t28530: F, t29637: F, t29669: F, t7467: F, t764: F, t9125: F, t9128: F) -> (F,) {
    let t29672 = t29636 * t12061;
    let t29685 = -t12002 - 0.2283111111111111111e-1 * t15989 + 0.11415555555555555555e-1 * t22564 - 0.34246666666666666665e-1 * t22575 + 0.17123333333333333333e-1 * t22583 - 0.19025925925925925925e-1 * t28371 + 0.68493333333333333331e-1 * t28375 - 0.34246666666666666665e-1 * t28379 - 0.10274e0 * t28383 + 0.10274e0 * t28387 - 0.17123333333333333333e-1 * t28391;
    let t29688 = 3.0 * t24785 * t2605 + 3.0 * t7467 * t9125 + 0.96494049533612093922e2 * t18546 * t9128 - 0.19298809906722418785e3 * t12018 * t29637 + 1.0 * t1966 * t29669 + 0.20691336878655965246e4 * t12059 * t29672 + t28530 - t28352 - t28354 - t28356 + t28360 - t28441 - t28444 + t28461 - t28464 - 0.3109e-1 * t29685 * t764;
    (t29688,)
}
