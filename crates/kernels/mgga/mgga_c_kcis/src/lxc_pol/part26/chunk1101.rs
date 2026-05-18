//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1101/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1101<F: Float>(t28426: F, t7898: F, t5628: F, t7931: F, t303: F, t1307: F, t28373: F, t3984: F, t5885: F, t5709: F) -> (F, F, F, F, F, F, F) {
    let t28427 = t7898 * t28426;
    let t28429 = t7931 * t5628;
    let t28430 = t303 * t28429;
    let t28438 = t28373 * t1307;
    let t28439 = t3984 * t28438;
    let t28442 = t5885 * t1307;
    let t28443 = t5709 * t28442;
    (t28427, t28429, t28430, t28438, t28439, t28442, t28443)
}
