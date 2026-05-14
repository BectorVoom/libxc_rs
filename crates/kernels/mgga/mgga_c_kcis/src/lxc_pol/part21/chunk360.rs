//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 360/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk360<F: Float>(t1268: F, t1856: F, t1240: F, t1272: F, t1715: F, t1751: F, t1770: F, t1776: F, t1844: F, t430: F, t1798: F, t1802: F, t1806: F, t1810: F, t1814: F, t1818: F) -> (F, F, F) {
    let t1857 = t1856 * t1268;
    let t1864 = t1844 * t430 - 0.66725e-1 * t1240 * t1857 + t1272 + 0.11607361111111111111e-2 * t1715 + 0.17411041666666666666e-2 * t1751 - 0.17411041666666666666e-2 * t1770 + 0.11607361111111111111e-2 * t1776;
    let t1872 = 0.9375e-1 * t1798 - 0.9375e-1 * t1802 + 0.625e-1 * t1806 - 0.101171875e-1 * t1810 + 0.101171875e-1 * t1814 - 0.13489583333333333333e-1 * t1818;
    (t1857, t1864, t1872)
}
