//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 734/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk734<F: Float>(t14498: F, t14506: F, t14511: F, t14515: F, t13004: F, t13005: F, t13234: F, t13237: F, t13243: F, t13245: F, t13839: F, t13841: F, t13951: F, t14491: F, t331: F, t748: F) -> (F, F) {
    let t14517 = t14498 + t14506 + t14511 + t14515;
    let t14519 = t14491 * t331 - t14517 * t748 + t13004 - t13005 - t13234 - t13237 + t13243 + t13245 + F::new(4.0) * t13839 - F::new(2.0) * t13841 - F::new(2.0) * t13951;
    (t14517, t14519)
}
