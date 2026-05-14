//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1250/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1250<F: Float>(t28827: F, t681: F, t89: F, t1212: F, t24980: F, t2789: F, t2862: F, t6318: F, t28831: F, t112443: F, t193: F, t2682: F, t113577: F, t113580: F, t113584: F, t113588: F, t113592: F, t113595: F, t113598: F, t99810: F) -> (F, F, F, F, F) {
    let t113601 = t89 * t681 * t28827;
    let t113602 = 4.0 / 3.0 * t113601;
    let t113606 = t24980 * t2862 * t6318 * t1212 * t2789;
    let t113609 = t89 * t681 * t28831;
    let t113610 = 4.0 / 3.0 * t113609;
    let t113613 = t89 * t193 * t112443 * t2682;
    let t113615 = 2.0 * t113577 + t113580 + 4.0 * t113584 + 2.0 * t113588 + t99810 + t113592 - t113595 - 2.0 / 3.0 * t113598 - t113602 - t113606 / 2.0 - t113610 - 6.0 * t113613;
    (t113601, t113606, t113609, t113613, t113615)
}
