//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 943/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk943(t12018: f64, t12059: f64, t18546: f64, t1966: f64, t24785: f64, t2605: f64, t28352: f64, t28354: f64, t28356: f64, t28360: f64, t28441: f64, t28444: f64, t28461: f64, t28464: f64, t28530: f64, t29637: f64, t29669: f64, t29672: f64, t29685: f64, t7467: f64, t764: f64, t9125: f64, t9128: f64) -> f64 {
    let t29688 = 3.0_f64 * t24785 * t2605 + 3.0_f64 * t7467 * t9125 + 0.96494049533612093922e2_f64 * t18546 * t9128 - 0.19298809906722418785e3_f64 * t12018 * t29637 + 1.0_f64 * t1966 * t29669 + 0.20691336878655965246e4_f64 * t12059 * t29672 + t28530 - t28352 - t28354 - t28356 + t28360 - t28441 - t28444 + t28461 - t28464 - 0.3109e-1_f64 * t29685 * t764;
    t29688
}
