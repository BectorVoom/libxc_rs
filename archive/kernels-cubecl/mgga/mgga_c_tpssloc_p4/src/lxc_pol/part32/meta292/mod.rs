//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1305;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1306;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta292<F: Float>(t2617: F, t2638: F, t116: F, t126: F, t136: F, t16: F, t2386: F, t625: F, t2385: F, t686: F, t781: F, t685: F, t120: F, t118: F, t123: F, t2397: F, t693: F, t119: F, t133: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9674, t9689, t9691, t9692, t9694, t9695) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1305::<F>(t2617, t2638, t116, t126, t136, t16, t2386, t625, t2385, t686, t781, t685);
        let (t9697, t9698, t9702, t9704, t9706, t9709) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1306::<F>(t120, t781, t118, t123, t116, t16, t2397, t9691, t693, t9694, t119, t133, t625);
    (t9674, t9689, t9692, t9695, t9697, t9698, t9702, t9704, t9706, t9709)
}
