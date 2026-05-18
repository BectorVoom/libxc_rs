//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1393/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1393<F: Float>(t102842: F, t102846: F, t102848: F, t102850: F, t102854: F, t102855: F, t102860: F, t102864: F, t103785: F, t103788: F, t103790: F, t103793: F, t103898: F, t103900: F, t103905: F, t103909: F, t103914: F, t103917: F, t103925: F) -> F {
    let t103935 = -t102842 + t102846 - t102848 + t102850 + t102854 - t102855 - t102860 + t102864 + t103785 + t103788 - t103790 - t103793 - t103898 + t103900 + t103905 + t103909 + t103914 - t103917 + t103925;
    t103935
}
