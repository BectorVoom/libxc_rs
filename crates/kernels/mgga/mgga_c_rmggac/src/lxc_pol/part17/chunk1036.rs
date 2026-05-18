//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1036/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1036<F: Float>(t1173: F, t674: F, t9824: F, t1997: F, t2320: F, t38967: F, t1971: F, t333: F, t7230: F, t880: F, t9969: F, t2144: F, t352: F) -> (F, F, F, F) {
    let t47029 = t9824 * t1173 * t674;
    let t47030 = t47029 * t1997;
    let t47032 = t38967 * t2320;
    let t47037 = t7230 * t1971 * t880 * t9969 * t333;
    let t47042 = t7230 * t1971 * t2144 * t9969 * t352;
    (t47030, t47032, t47037, t47042)
}
