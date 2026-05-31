//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 943/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk943<F: Float>(t12018: F, t12059: F, t18546: F, t1966: F, t24785: F, t2605: F, t28352: F, t28354: F, t28356: F, t28360: F, t28441: F, t28444: F, t28461: F, t28464: F, t28530: F, t29637: F, t29669: F, t29672: F, t29685: F, t7467: F, t764: F, t9125: F, t9128: F) -> F {
    let t29688 = F::cast_from(3.0_f64) * t24785 * t2605 + F::cast_from(3.0_f64) * t7467 * t9125 + F::cast_from(0.96494049533612093922e2_f64) * t18546 * t9128 - F::cast_from(0.19298809906722418785e3_f64) * t12018 * t29637 + F::cast_from(1.0_f64) * t1966 * t29669 + F::cast_from(0.20691336878655965246e4_f64) * t12059 * t29672 + t28530 - t28352 - t28354 - t28356 + t28360 - t28441 - t28444 + t28461 - t28464 - F::cast_from(0.3109e-1_f64) * t29685 * t764;
    t29688
}
