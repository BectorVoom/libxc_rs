//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 481/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk481<F: Float>(t3147: F, t479: F, t471: F, t3153: F, t1121: F, t414: F, t66: F, t474: F, t3089: F, t1285: F, t1264: F, t828: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3598 = t479 * t3147;
    let t3603 = t471 * t471;
    let t3604 = t3153 * t3603;
    let t3617 = F::cast_from(1.0_f64) / t414 / t1121;
    let t3618 = t66 * t3617;
    let t3623 = t474 * t479;
    let t3624 = t3623 * t3089;
    let t3625 = t1285 * t3624;
    let t3626 = t828 * t1264;
    (t3598, t3603, t3604, t3617, t3618, t3623, t3624, t3625, t3626)
}
