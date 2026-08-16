//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 956/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk956<F: Float>(t119: F, t20356: F, t210: F, t1810: F, t6347: F, t11982: F, t11984: F, t20354: F, t20355: F, t20360: F, t20361: F, t20365: F, t20366: F, t20370: F, t9457: F, t9476: F, t9484: F) -> (F, F, F) {
    let t20511 = t119 * t20356;
    let t20512 = t210 * t20511;
    let t20516 = t210 * t1810 * t6347;
    let t20519 = -t20354 - t9457 + t20355 + t9476 + t9484 - t20360 - t20361 + t11982 - t20365 - t20366 - t11984 - t20370;
    (t20512, t20516, t20519)
}
