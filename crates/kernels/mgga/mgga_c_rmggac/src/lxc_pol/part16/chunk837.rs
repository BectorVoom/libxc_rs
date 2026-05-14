//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 837/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk837<F: Float>(t1986: F, t6590: F, t675: F, t2289: F, t9087: F, t2412: F, t8592: F, t2410: F, t3350: F, t8515: F, t8519: F, t39277: F, t8668: F, t8831: F, t1987: F, t45561: F) -> (F, F, F, F, F, F, F) {
    let t45742 = t675 * t1986 * t6590;
    let t45744 = t9087 * t2289;
    let t45746 = t2412 * t8592;
    let t45750 = t2410 * t8515 * t3350 * t8519;
    let t45752 = t39277 * t8668;
    let t45754 = t39277 * t8831;
    let t45757 = t45561 * t1987;
    (t45742, t45744, t45746, t45750, t45752, t45754, t45757)
}
