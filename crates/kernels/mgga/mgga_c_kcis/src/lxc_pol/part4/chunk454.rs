//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 454/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk454<F: Float>(t1911: F, t482: F, t1349: F, t1352: F, t1891: F, t1898: F, t1901: F, t1904: F) -> (F, F) {
    let t1912 = t1911 * t482;
    let t1919 = 0.258925e1 * t1898 - t1349 - 0.301925e0 * t1891 + 0.16504875e0 * t1901 - t1352 - 0.82785e-1 * t1904;
    (t1912, t1919)
}
