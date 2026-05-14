//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1343/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1343<F: Float>(t117874: F, t9740: F, t112858: F, t34533: F, t10487: F, t2029: F, t33207: F, t9990: F, t117120: F, t11986: F, t18325: F, t780: F, t33162: F, t34573: F, t34484: F, t9733: F) -> (F, F, F, F, F, F, F, F) {
    let t118393 = 0.11574074074074074074e-2 * t9740 * t117874;
    let t118405 = 0.11574074074074074074e-2 * t9740 * t112858 * t34533;
    let t118412 = t2029 * t10487;
    let t118419 = t9990 * t33207;
    let t118439 = 0.10317654320987654321e-2 * t117120;
    let t118443 = t11986 * t780 * t18325;
    let t118450 = t34573 * t33162;
    let t118455 = 0.34722222222222222222e-2 * t9733 * t34484;
    (t118393, t118405, t118412, t118419, t118439, t118443, t118450, t118455)
}
