//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3151/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3151<F: Float>(t17728: F, t3555: F, t489: F, t12772: F, t17736: F, t17738: F, t3623: F, t53739: F, t13127: F, t12865: F, t3746: F, t12831: F, t17395: F) -> (F, F, F, F, F, F) {
    let t56861 = t3555 * t489 * t17728;
    let t56867 = t17736 * t12772 * t17738;
    let t56878 = t3623 * t53739;
    let t56879 = t13127 * t56878;
    let t56888 = t3746 * t12865;
    let t56953 = t12831 * t17395;
    (t56861, t56867, t56878, t56879, t56888, t56953)
}
