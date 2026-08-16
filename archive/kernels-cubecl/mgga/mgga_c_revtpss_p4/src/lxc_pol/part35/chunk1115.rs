//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1115/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1115<F: Float>(t10867: F, t867: F, t25410: F, t93189: F, t25374: F, t93169: F, t2453: F, t555: F, t25898: F, t25304: F, t2482: F, t7262: F, t814: F) -> (F, F, F, F, F, F, F, F) {
    let t93355 = t867 * t10867;
    let t93371 = t93189 * t25410;
    let t93377 = t93169 * t25374;
    let t94382 = t2453 * t555;
    let t94383 = t94382 * t25898;
    let t94390 = t25304 * t555;
    let t94391 = t94390 * t25898;
    let t94423 = t2482 * t7262 * t814;
    (t93355, t93371, t93377, t94382, t94383, t94390, t94391, t94423)
}
