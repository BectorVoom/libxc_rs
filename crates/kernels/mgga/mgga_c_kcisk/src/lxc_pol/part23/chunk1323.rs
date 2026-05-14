//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1323/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1323<F: Float>(t111274: F, t111306: F, t111338: F, t111364: F, t111388: F, t111414: F, t111439: F, t111468: F, t218: F, t110824: F, t110827: F, t110829: F, t110832: F, t110834: F, t110837: F, t110840: F, t110842: F, t110845: F, t110847: F, t110849: F, t110851: F, t110854: F, t110856: F) -> (F, F) {
    let t111472 = (t111274 + t111306 + t111338 + t111364 + t111388 + t111414 + t111439 + t111468) * t218;
    let t111490 = -3.0 / 16.0 * t110824 + t110827 / 32.0 + 3.0 / 32.0 * t110829 + 3.0 / 16.0 * t110832 + 3.0 / 32.0 * t110834 - 3.0 / 16.0 * t110837 - 3.0 / 8.0 * t110840 + 3.0 / 4.0 * t110842 - t110845 / 8.0 + 3.0 / 8.0 * t110847 - 3.0 / 8.0 * t110849 + 3.0 / 4.0 * t110851 - 3.0 / 2.0 * t110854 + 3.0 / 2.0 * t110856;
    (t111472, t111490)
}
