//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 974/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk974<F: Float>(t13218: F, t1423: F, t5171: F, t1631: F, t1887: F, t3047: F, t802: F, t10040: F, t10046: F, t13205: F, t13207: F, t13210: F, t13212: F, t13214: F, t13216: F, t1512: F, t1928: F) -> (F, F, F, F, F, F, F, F) {
    let t13219 = 2.0 / 9.0 * t13218;
    let t13220 = t1423 * t5171;
    let t13221 = 2.0 / 45.0 * t13220;
    let t13223 = t1887 * t1631 / 10.0;
    let t13225 = t802 * t3047 / 10.0;
    let t13226 = t10040 / 15.0;
    let t13227 = t10046 / 45.0;
    let t13228 = t13205 + t13207 + t13210 + t13212 + t13214 + t13216 - t13219 + t13221 - t13223 - t13225 - t13226 + t13227;
    let t13230 = t1512 * t1928;
    (t13219, t13221, t13223, t13225, t13226, t13227, t13228, t13230)
}
