//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 368/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk368<F: Float>(t1636: F, t1842: F, t695: F, t967: F, t227: F, t694: F) -> (F, F, F, F, F) {
    let t1843 = t1842 * t1636;
    let t1846 = t967 * t695;
    let t1847 = F::cast_from(0.5179538907796306876e-4_f64) * t1846;
    let t1848 = t694 * t227;
    let t1849 = F::cast_from(1.0_f64) / t1848;
    (t1843, t1846, t1847, t1848, t1849)
}
