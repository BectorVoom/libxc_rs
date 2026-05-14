//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1066/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1066<F: Float>(t94534: F, t94537: F, t94540: F, t94542: F, t94546: F, t94548: F, t94552: F, t94554: F, t94557: F, t94559: F, t94561: F, t94565: F, t96358: F, t96359: F, t96314: F, t96329: F, t96345: F) -> (F,) {
    let t96360 = -0.17149607247227894789e-2 * t94534 + 0.30492001685571196935e-4 * t94537 - 0.2168591159877823526e-3 * t94540 - 0.6098400337114239387e-3 * t94542 - 0.27210710165601593065e0 * t94546 + 0.48018900292238105409e-1 * t94548 - 0.17149607247227894789e-3 * t94552 - 0.91464571985215438874e-3 * t94554 + 0.85748036236139473944e-4 * t94557 - 0.24009450146119052704e0 * t94559 + 0.30492001685571196935e-2 * t94561 - 0.54214778996945588151e-4 * t94565 - t96358 - t96359;
    let t96362 = t96314 + t96329 + t96345 + t96360;
    (t96362,)
}
