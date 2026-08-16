//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1996/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1996<F: Float>(t25374: F, t93320: F, t25410: F, t93189: F, t93174: F, t93341: F, t93169: F, t93191: F, t2439: F, t7048: F, t780: F, t785: F) -> (F, F, F, F, F, F, F) {
    let t93364 = t93320 * t25374;
    let t93371 = t93189 * t25410;
    let t93372 = t93371 * t93174;
    let t93374 = t93341 * t25410;
    let t93377 = t93169 * t25374;
    let t93378 = t93377 * t93191;
    let t93382 = t2439 * t785 * t7048 * t780;
    (t93364, t93371, t93372, t93374, t93377, t93378, t93382)
}
