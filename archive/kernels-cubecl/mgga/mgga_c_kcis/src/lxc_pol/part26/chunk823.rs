//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 823/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk823<F: Float>(t14954: F, t85: F, t119: F, t41: F, t12274: F, t2003: F, t6019: F, sigma2: F) -> (F, F, F, F) {
    let t14955 = t85 * t14954;
    let t15007 = t119 * t41;
    let t15008 = t85 * t15007;
    let t15800 = t12274 * t2003;
    let t15808 = t6019 * sigma2;
    (t14955, t15008, t15800, t15808)
}
