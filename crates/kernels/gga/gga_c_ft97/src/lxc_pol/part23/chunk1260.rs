//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1260/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1260<F: Float>(t24543: F, t30983: F, t122330: F, t24438: F, t6118: F, t2354: F, t24546: F, t4969: F, t123814: F, t27: F, t676: F, t89: F, t27787: F, t27805: F, t3746: F, t27468: F, t3875: F) -> (F, F, F, F, F, F) {
    let t124183 = t24543 * t30983;
    let t124186 = t6118 * t24438 * t122330;
    let t124190 = t6118 * t2354 * t24546 * t4969;
    let t124194 = t89 * t27 * t676 * t123814;
    let t124198 = t27805 * t2354 * t27787 * t3746;
    let t124200 = t27468 * t3875;
    (t124183, t124186, t124190, t124194, t124198, t124200)
}
