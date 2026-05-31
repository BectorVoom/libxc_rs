//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 354/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk354<F: Float>(t1260: F, t1851: F, t286: F, t1249: F, t1251: F, t1847: F) -> (F, F, F) {
    let t1852 = t1260 * t1851;
    let t1853 = t286 * t1852;
    let t1856 = t1249 + t1251 * t1847 / F::cast_from(576.0_f64) - t1251 * t1853 / F::cast_from(192.0_f64);
    (t1852, t1853, t1856)
}
