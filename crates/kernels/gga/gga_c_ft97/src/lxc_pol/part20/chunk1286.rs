//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1286/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1286<F: Float>(t29223: F, t8392: F, t29226: F, t29229: F, t7107: F, t8232: F, t10703: F, t1091: F, t113066: F, t113467: F, t15299: F, t1901: F, t2405: F, t2413: F, t24873: F, t28516: F, t2881: F, t29307: F, t3281: F, t4299: F, t44518: F, t446: F, t57180: F, t6260: F, t684: F, t7045: F, t835: F, t840: F, t871: F, t99925: F, t99938: F, t99948: F) -> (F,) {
    let t114938 = 2.0 / 27.0 * t8392 * t29223;
    let t114940 = 4.0 / 27.0 * t8392 * t29226;
    let t114942 = 4.0 / 81.0 * t8392 * t29229;
    let t114979 = t8232 * t7107;
    let t114981 = t114938 + t114940 - t114942 - 4.0 / 9.0 * t1901 * t57180 * t28516 - 4.0 / 9.0 * t1901 * t15299 * t113066 - 2.0 / 27.0 * t1901 * t44518 * t7045 * t2405 - t1901 * t10703 * t7045 * t2413 / 9.0 - 2.0 / 9.0 * t1901 * t15299 * t113467 - 4.0 / 9.0 * t99925 + 2.0 / 3.0 * t446 * t840 * t871 * t6260 * t4299 + t99938 / 9.0 + t1901 * t2881 * t99948 * t1091 / 9.0 - 2.0 / 9.0 * t1901 * t10703 * t29307 * t684 - 2.0 / 9.0 * t3281 * t835 * t871 * t24873 + 4.0 / 27.0 * t114979;
    (t114981,)
}
