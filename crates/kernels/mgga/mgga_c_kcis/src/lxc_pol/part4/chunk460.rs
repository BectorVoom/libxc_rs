//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 460/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk460<F: Float>(t1260: F, t1851: F, t286: F, t1249: F, t1251: F, t1847: F, t1268: F, t1240: F, t1272: F, t1715: F, t1751: F, t1770: F, t1776: F, t1844: F, t430: F) -> (F, F, F, F, F) {
    let t1852 = t1260 * t1851;
    let t1853 = t286 * t1852;
    let t1856 = t1249 + t1251 * t1847 / F::new(576.0) - t1251 * t1853 / F::new(192.0);
    let t1857 = t1856 * t1268;
    let t1864 = t1844 * t430 - F::new(0.66725e-1) * t1240 * t1857 + t1272 + F::cast_from(0.11607361111111111111e-2_f64) * t1715 + F::cast_from(0.17411041666666666666e-2_f64) * t1751 - F::cast_from(0.17411041666666666666e-2_f64) * t1770 + F::cast_from(0.11607361111111111111e-2_f64) * t1776;
    (t1852, t1853, t1856, t1857, t1864)
}
