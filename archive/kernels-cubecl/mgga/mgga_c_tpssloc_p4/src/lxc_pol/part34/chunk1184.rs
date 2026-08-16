//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1184/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1184<F: Float>(t20347: F, t89: F, t191: F, t192: F, t20350: F, t5445: F, t72: F, t7431: F, t20284: F, t71: F, t33: F, t75284: F) -> (F, F, F, F, F) {
    let t106734 = t89 * t20347;
    let t106755 = t20350 * t191 * t192;
    let t106758 = t72 * t7431 * t5445;
    let t106800 = t71 * t20284;
    let t106804 = t75284 * t33;
    (t106734, t106755, t106758, t106800, t106804)
}
