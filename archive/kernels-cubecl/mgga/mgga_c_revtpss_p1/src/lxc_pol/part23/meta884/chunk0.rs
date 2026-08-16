//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2796/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2796<F: Float>(t22307: F, t545: F, t689: F, t869: F, t14239: F, t14242: F, t10023: F, t22314: F, t2470: F, t13790: F, t5658: F, t10022: F, t2782: F) -> (F, F, F, F) {
    let t75174 = t689 * t869 * t545 * t22307;
    let t75176 = t14239 * t14242;
    let t75179 = t10023 * t22314 * t2470;
    let t75188 = t13790 * t5658;
    let t75190 = t2782 * t10022 * t75188;
    (t75174, t75176, t75179, t75190)
}
