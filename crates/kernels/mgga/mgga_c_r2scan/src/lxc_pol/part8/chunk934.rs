//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 934/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk934<F: Float>(t2124: F, t2551: F, t8837: F, t2892: F, t537: F, t495: F, t3052: F) -> (F, F, F, F) {
    let t8839 = t2124 * t8837 * t2551;
    let t8842 = t537 * t2892;
    let t8844 = t2124 * t8842 * t495;
    let t8847 = t537 * t3052;
    (t8839, t8842, t8844, t8847)
}
