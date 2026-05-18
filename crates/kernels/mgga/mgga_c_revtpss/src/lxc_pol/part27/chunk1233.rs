//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1233/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1233<F: Float>(t25378: F, t93364: F, t25387: F, t93330: F, t25410: F, t93189: F, t93174: F, t93341: F, t25413: F, t25374: F, t93169: F, t93191: F) -> (F, F, F, F, F) {
    let t93365 = t93364 * t25378;
    let t93369 = t25387 * t93330;
    let t93371 = t93189 * t25410;
    let t93372 = t93371 * t93174;
    let t93374 = t93341 * t25410;
    let t93375 = t93374 * t25413;
    let t93377 = t93169 * t25374;
    let t93378 = t93377 * t93191;
    (t93365, t93369, t93372, t93375, t93378)
}
