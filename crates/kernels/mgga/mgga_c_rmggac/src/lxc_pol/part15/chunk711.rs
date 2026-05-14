//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 711/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk711<F: Float>(t8647: F, t8651: F, t8679: F, t8685: F, t8690: F, t8796: F, t8802: F, t8805: F, t8809: F, t8813: F, t8815: F, t8818: F, t2019: F, t2323: F, t7926: F, t7487: F, t8346: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38279 = 0.81823984962736025184e-1 * t8647;
    let t38280 = 0.20455996240684006296e-1 * t8651;
    let t38292 = 0.85129199786595678796e-5 * t8679;
    let t38295 = 0.85129199786595678796e-5 * t8685;
    let t38296 = 0.85129199786595678796e-5 * t8690;
    let t38300 = 0.39914139006212695214e-1 * t8796;
    let t38304 = 0.79828278012425390428e-1 * t8802;
    let t38305 = 0.79828278012425390428e-1 * t8805;
    let t38306 = 0.10215503974391481455e-3 * t8809;
    let t38307 = 0.25538759935978703638e-4 * t8813;
    let t38308 = 0.25538759935978703638e-4 * t8815;
    let t38310 = 0.4726e1 * t8818;
    let t38312 = t2019 * t7926 * t2323;
    let t38314 = t7487 * t8346;
    (t38279, t38280, t38292, t38295, t38296, t38300, t38304, t38305, t38306, t38307, t38308, t38310, t38312, t38314)
}
