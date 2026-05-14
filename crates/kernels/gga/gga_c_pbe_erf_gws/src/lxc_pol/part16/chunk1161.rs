//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1161/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1161<F: Float>(t54052: F, t54072: F, t54043: F, t54045: F, t54048: F, t54057: F, t54059: F, t54061: F, t54063: F, t54065: F, t54067: F, t54069: F, t54087: F, t54094: F, t54102: F, t51244: F, t54075: F, t54077: F, t54080: F, t54082: F, t54085: F, t54092: F, t54096: F, t54098: F) -> (F, F) {
    let t55452 = 7.0 / 96.0 * t54052;
    let t55460 = 7.0 / 72.0 * t54072;
    let t55461 = t54043 / 12.0 + t54045 / 192.0 + t54048 / 32.0 - t55452 - t54057 / 4.0 - 5.0 / 96.0 * t54059 + t54061 / 48.0 + t54063 / 192.0 - t54065 / 96.0 + t54067 / 96.0 - t54069 / 16.0 + t55460;
    let t55467 = 7.0 / 72.0 * t54087;
    let t55469 = 35.0 / 216.0 * t54094;
    let t55473 = 7.0 / 36.0 * t54102;
    let t55474 = -t54075 / 24.0 + t54077 / 384.0 - t54080 / 24.0 + t54082 / 24.0 - t54085 / 24.0 + t55467 - t54092 / 6.0 + t55469 - t54096 / 384.0 + t54098 / 64.0 - 7.0 / 144.0 * t51244 + t55473;
    (t55461, t55474)
}
