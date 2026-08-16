//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1184/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1184(t20347: f64, t89: f64, t191: f64, t192: f64, t20350: f64, t5445: f64, t72: f64, t7431: f64, t20284: f64, t71: f64, t33: f64, t75284: f64) -> (f64, f64, f64, f64, f64) {
    let t106734 = t89 * t20347;
    let t106755 = t20350 * t191 * t192;
    let t106758 = t72 * t7431 * t5445;
    let t106800 = t71 * t20284;
    let t106804 = t75284 * t33;
    (t106734, t106755, t106758, t106800, t106804)
}
