//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1228/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1228<F: Float>(t113238: F, t2665: F, t6317: F, t684: F, t28743: F, t99312: F, t113224: F, t113227: F, t113231: F, t113236: F, t99317: F, t99739: F, t99741: F, t99742: F, t99743: F, t24980: F, t25165: F, t2862: F, t28741: F) -> (F, F, F, F) {
    let t113241 = t6317 * t2665 * t113238 * t684;
    let t113243 = t99312 * t28743;
    let t113244 = t113243 / 3.0;
    let t113245 = -2.0 / 3.0 * t113224 + t113227 + 8.0 / 27.0 * t99317 + t99739 - t99741 - t99742 + t99743 + t113231 / 6.0 - t113236 / 2.0 + t113241 / 3.0 + t113244;
    let t113248 = t24980 * t2862 * t25165 * t28741;
    (t113241, t113243, t113245, t113248)
}
