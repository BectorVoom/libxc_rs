//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 637/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk637<F: Float>(t13041: F, t1445: F, t833: F, t2097: F, t3039: F, t3277: F, t13010: F, t13015: F, t13018: F, t13021: F, t13026: F, t13028: F, t13029: F, t13031: F, t13036: F, t13040: F) -> (F, F, F) {
    let t13042 = t1445 * t13041;
    let t13044 = 0.43710935587469654631e2 * t833 * t13042;
    let t13045 = t3039 * t2097;
    let t13047 = 0.25025342966295298669e1 * t3277 * t13045;
    let t13048 = -0.13803453343411469884e2 * t13010 - t13015 - t13018 + 0.14300195980740170668e1 * t13021 + t13026 + t13028 + 0.71500979903700853338e0 * t13029 - 0.21450293971110256002e1 * t13031 + t13036 - t13040 + t13044 - t13047;
    (t13042, t13045, t13048)
}
