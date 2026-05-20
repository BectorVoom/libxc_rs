//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3132/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3132<F: Float>(t12855: F, t12916: F, t17455: F, t3584: F, t5333: F, t1222: F, t16738: F, t17240: F, t16742: F, t16733: F, t13036: F, t13039: F, t57403: F) -> (F, F, F, F, F, F) {
    let t57735 = t12855 * t12916 * t17455;
    let t57737 = t5333 * t3584;
    let t57743 = t1222 * t17240 * t16738;
    let t57746 = t1222 * t17240 * t16742;
    let t57749 = t1222 * t17240 * t16733;
    let t57759 = t13036 * t13039 * t57403;
    (t57735, t57737, t57743, t57746, t57749, t57759)
}
