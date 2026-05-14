//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1126/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1126<F: Float>(t15125: F, t15168: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15163: F, t15166: F, t15170: F, t15173: F, t15191: F, t15197: F, t11134: F, t11136: F, t11138: F, t11140: F, t11334: F, t11338: F, t11339: F, t11366: F, t11368: F, t15221: F, t15230: F) -> (F, F, F, F, F) {
    let t15435 = 0.39862222222222222222e0 * t15125;
    let t15447 = 0.21908444444444444444e0 * t15168;
    let t15450 = -0.19931111111111111111e0 * t15137 - 0.33218518518518518518e0 * t15142 + 0.11958666666666666667e1 * t15147 + 0.59793333333333333334e0 * t15151 + 0.11958666666666666667e1 * t15156 - 0.17938e1 * t15160 + 0.16431333333333333333e0 * t15163 - 0.49293999999999999999e0 * t15166 - t15447 + 0.36514074074074074074e-1 * t15170 - 0.54771111111111111112e-1 * t15173;
    let t15457 = 0.19931111111111111111e0 * t15191;
    let t15459 = 0.10954222222222222222e0 * t15197;
    let t15472 = -t11334 - t11338 + 0.3071625e0 * t15221 + 0.18257037037037037037e-1 * t11339 - 0.19931111111111111111e0 * t11138 - 0.26574814814814814816e0 * t11134 + 0.99655555555555555557e-1 * t11140 + 0.66437037037037037038e-1 * t11136 - 0.18257037037037037037e0 * t11366 + 0.54771111111111111111e-1 * t11368 + 0.1898925e1 * t15230;
    (t15435, t15450, t15457, t15459, t15472)
}
