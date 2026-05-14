//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 669/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk669<F: Float>(t4971: F, t654: F, t11154: F, t1800: F, t1869: F, t4597: F, t642: F, t1757: F, t3290: F, t5192: F, t6674: F, t1870: F, t704: F, t1894: F, t5063: F, t1801: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t11213 = t654 * t4971;
    let t11214 = t11213 * t11154;
    let t11215 = t1800 * t11214;
    let t11216 = t1869 * t11215;
    let t11218 = t642 * t4597;
    let t11219 = t3290 * t1757;
    let t11220 = t11218 * t11219;
    let t11221 = t5192 * t11220;
    let t11222 = t6674 * t11221;
    let t11224 = t1870 * t1870;
    let t11225 = 1.0 / t11224;
    let t11226 = t704 * t11225;
    let t11227 = t11226 * sigma2;
    let t11228 = t5063 * t1894;
    let t11229 = t1801 * t11228;
    (t11216, t11219, t11222, t11225, t11226, t11227, t11228, t11229)
}
