//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1124/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1124<F: Float>(t22813: F, t828: F, t9942: F, t1414: F, t22809: F, t22079: F, t3936: F, t6869: F, t13790: F, t5673: F, t1883: F, t22074: F) -> (F, F, F, F, F) {
    let t22815 = t9942 * t828 * t22813;
    let t22822 = t1414 * t828 * t22809;
    let t22829 = t3936 * t22079 * t6869;
    let t22833 = t5673 * t22079 * t13790;
    let t22837 = t3936 * t22074 * t1883;
    (t22815, t22822, t22829, t22833, t22837)
}
