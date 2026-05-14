//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1353/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1353<F: Float>(t113350: F, t6328: F, t33643: F, t6313: F, t19848: F, t32277: F, t6370: F, t1415: F, t27411: F, t113369: F, t9839: F, t27092: F, t9491: F, t113378: F, t25383: F, t500: F, t79161: F) -> (F, F, F, F, F, F, F, F) {
    let t119753 = t113350 * t6328;
    let t119755 = t33643 * t6313;
    let t119758 = t19848 * t32277 * t6370;
    let t119760 = t1415 * t27411;
    let t119762 = t113369 * t9839;
    let t119764 = t9491 * t27092;
    let t119766 = t113378 * t25383;
    let t119768 = t79161 * t500;
    (t119753, t119755, t119758, t119760, t119762, t119764, t119766, t119768)
}
