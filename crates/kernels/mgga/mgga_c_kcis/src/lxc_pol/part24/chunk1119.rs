//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1119/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1119<F: Float>(t142: F, t164: F, t9273: F, t113: F, t8750: F, t898: F, t9005: F, t2764: F, t2770: F, t895: F, t9016: F, t224: F, t227: F, t9015: F) -> (F, F, F, F, F, F) {
    let t35635 = t142 / t9273 / t164;
    let t36222 = t113 * t8750;
    let t36429 = t9005 * t898;
    let t36436 = t2764 * t2770;
    let t36439 = t895 * t9016;
    let t36513 = t224 / t9015 / t227;
    (t35635, t36222, t36429, t36436, t36439, t36513)
}
