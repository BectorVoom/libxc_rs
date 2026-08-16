//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1059/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1059(t74594: f64, t74616: f64, t68739: f64, t74598: f64, t74600: f64, t74603: f64, t74605: f64, t74609: f64, t74610: f64, t77117: f64, t77119: f64, t77121: f64, t77123: f64, t77125: f64, t77127: f64, t77129: f64, t77132: f64) -> f64 {
    let t80136 = 0.15372131649401827112e-4_f64 * t74594;
    let t80138 = 0.49700494569958178262e-1_f64 * t74616;
    let t80139 = -t80136 + t77117 + t77119 - t74598 - t74600 - t74603 - t74605 - t74609 + t77121 - 0.31062809106223861414e-2_f64 * t74610 + t68739 + t77123 - t77125 - t77127 - t77129 - t80138 - t77132;
    t80139
}
