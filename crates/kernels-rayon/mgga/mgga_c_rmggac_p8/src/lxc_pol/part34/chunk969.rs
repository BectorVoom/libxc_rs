//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 969/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk969(t77118: f64, t74557: f64, t74569: f64, t74587: f64, t74594: f64, t74598: f64, t74600: f64, t74603: f64, t74605: f64, t77105: f64, t77107: f64, t77108: f64, t77109: f64, t77110: f64, t77111: f64, t77113: f64, t77117: f64) -> f64 {
    let t77119 = 0.42564599893297839398e-5_f64 * t77118;
    let t77120 = 0.17451485956252114154e-4_f64 * t74557 + t77105 + 0.10511583655740820313e-5_f64 * t74569 + t77107 - t77108 - t77109 - t77110 - t77111 + 0.17519306092901367188e-6_f64 * t74587 - t77113 - 0.15372131649401827111e-4_f64 * t74594 + t77117 + t77119 - t74598 - t74600 - t74603 - t74605;
    t77120
}
