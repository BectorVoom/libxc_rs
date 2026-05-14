//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 447/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk447<F: Float>(t1260: F, t1851: F, t286: F, t1249: F, t1251: F, t1847: F, t1268: F, t1240: F, t1272: F, t1715: F, t1751: F, t1770: F, t1776: F, t1844: F, t430: F, t1798: F, t1802: F, t1806: F, t1810: F, t1814: F, t1818: F) -> (F, F, F, F, F, F) {
    let t1852 = t1260 * t1851;
    let t1853 = t286 * t1852;
    let t1856 = t1249 + t1251 * t1847 / 576.0 - t1251 * t1853 / 192.0;
    let t1857 = t1856 * t1268;
    let t1864 = t1844 * t430 - 0.66725e-1 * t1240 * t1857 + t1272 + 0.11607361111111111111e-2 * t1715 + 0.17411041666666666666e-2 * t1751 - 0.17411041666666666666e-2 * t1770 + 0.11607361111111111111e-2 * t1776;
    let t1872 = 0.9375e-1 * t1798 - 0.9375e-1 * t1802 + 0.625e-1 * t1806 - 0.101171875e-1 * t1810 + 0.101171875e-1 * t1814 - 0.13489583333333333333e-1 * t1818;
    (t1852, t1853, t1856, t1857, t1864, t1872)
}
