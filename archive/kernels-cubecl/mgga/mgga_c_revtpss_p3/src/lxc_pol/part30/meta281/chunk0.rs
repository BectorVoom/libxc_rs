//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1233/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1233<F: Float>(t572: F, t7953: F, t1469: F, t1479: F, t61: F, t6971: F, t7571: F, t72: F, t1927: F) -> (F, F, F, F) {
    let t7955 = F::cast_from(3.0_f64) * t572 * t7953;
    let t8142 = -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1479 * t61 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7571 * t1469 + t6971;
    let t8143 = t8142 * t72;
    let t8144 = t8143 * t1927;
    (t7955, t8142, t8143, t8144)
}
