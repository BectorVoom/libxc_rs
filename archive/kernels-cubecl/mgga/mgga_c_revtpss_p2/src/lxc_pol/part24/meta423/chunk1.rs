//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1372/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1372<F: Float>(t44545: F, t828: F, t3566: F, t3766: F, t5330: F, t1209: F, t13141: F, t17708: F, t371: F, t481: F, t482: F, t9291: F) -> (F, F, F, F) {
    let t44546 = t828 * t44545;
    let t44550 = t3566 * t3766;
    let t44551 = t44550 * t5330;
    let t44578 = t1209 * t13141 * t17708;
    let t44607 = F::cast_from(0.14820648238345094262e-3_f64) * t481 * t371 * t9291 * t482;
    (t44546, t44551, t44578, t44607)
}
