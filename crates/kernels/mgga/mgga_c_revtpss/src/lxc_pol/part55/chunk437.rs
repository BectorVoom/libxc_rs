//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 437/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk437<F: Float>(t2674: F, t2677: F, t26: F, t66: F, t240: F, t243: F, t247: F, t237: F, t124: F, t212: F, t596: F, t800: F, t810: F, t775: F, t854: F, t236: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2678 = t2674 * t2677;
    let t2681 = 1.0 / t66 / t26;
    let t2682 = t2681 * t240;
    let t2684 = t2682 * t243 * t247;
    let t2686 = 0.56688979511669985553e-2 * t237 * t2684;
    let t2689 = t800 * t124 * t596 * t212;
    let t2691 = 0.76220476654346199061e-4 * t2689 * t810;
    let t2693 = t854 * t775;
    let t2694 = t236 * t2693;
    (t2678, t2681, t2682, t2684, t2686, t2689, t2691, t2693, t2694)
}
