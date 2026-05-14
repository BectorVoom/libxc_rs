//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1378/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1378<F: Float>(t113069: F, t116958: F, t117654: F, t117796: F, t117921: F, t117931: F, t117937: F, t118037: F, t33196: F, t33208: F, t33240: F, t33297: F, t34412: F, t34419: F, t34469: F, t34534: F, t34548: F, t34563: F, t9733: F, t9740: F) -> (F,) {
    let t118309 = 0.80416666666666666669e-2 * t33196 * t117654 + 0.34722222222222222222e-2 * t33208 * t34548 + 0.34722222222222222222e-2 * t33208 * t34534 + 0.67013888888888888888e-3 * t33196 * t117921 - 0.38801041666666666666e-3 * t34419 * t117931 - 0.26805555555555555556e-2 * t33196 * t117937 - 0.46296296296296296296e-2 * t33297 * t34563 - 0.46296296296296296296e-2 * t33208 * t34563 + 0.10416666666666666667e-1 * t9733 * t34469 + 0.116403125e-2 * t34419 * t118037 - 0.52083333333333333333e-2 * t9740 * t117796 - 0.92592592592592592594e-2 * t34412 * t33240 - 0.89351851851851851851e-3 * t113069 + 0.69644166666666666666e-2 * t116958;
    (t118309,)
}
