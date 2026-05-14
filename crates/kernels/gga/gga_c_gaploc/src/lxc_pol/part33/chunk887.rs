//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 887/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk887<F: Float>(t10646: F, t9664: F, t9666: F, t9669: F, t9672: F, t9674: F, t9676: F, t471: F, t3427: F, t64: F, t10627: F, t688: F, t779: F, t2508: F, t2554: F, t2932: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10647 = 0.42725145723012357132e-3 * t10646;
    let t10657 = -21.0 / 256.0 * t9664 + 147.0 / 8192.0 * t9666 - 63.0 / 524288.0 * t9669 + 21.0 / 524288.0 * t9672 - 49.0 / 8192.0 * t9674 + 7.0 / 256.0 * t9676;
    let t10658 = t10657 * t471;
    let t10660 = 4.0 / 3.0 * t3427 * t64;
    let t10663 = 7.0 / 256.0 * t9664;
    let t10664 = 21.0 / 8192.0 * t9666;
    let t10665 = 7.0 / 8192.0 * t9674;
    let t10666 = 7.0 / 768.0 * t9676;
    let t10682 = t10627 * t688;
    let t10683 = t779 * t10682;
    let t10685 = 0.76905262301422242837e-2 * t2508 * t10683;
    let t10691 = t2932 * t2554;
    (t10647, t10657, t10658, t10660, t10663, t10664, t10665, t10666, t10682, t10683, t10685, t10691)
}
