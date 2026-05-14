//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 672/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk672<F: Float>(t1803: F, t515: F, t996: F, t1504: F, t493: F, t1928: F, t435: F, t2941: F, t2902: F, t4538: F, t2942: F, t2894: F, t426: F, t1560: F, t173: F, t1559: F) -> (F, F, F, F, F, F) {
    let t8308 = t1803 * t515;
    let t8309 = t996 * t8308;
    let t8310 = t493 * t1504;
    let t8311 = t8309 * t8310;
    let t8313 = t435 * t1928;
    let t8314 = t2941 * t8313;
    let t8316 = t2902 * t4538;
    let t8317 = t8316 * t2942;
    let t8319 = t426 * t2894;
    let t8321 = t1560 * t173;
    let t8322 = t1559 * t8321;
    (t8311, t8314, t8316, t8317, t8319, t8322)
}
