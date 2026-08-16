//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 701/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk701<F: Float>(t2957: F, t8300: F, t2937: F, t4893: F, t1268: F, t991: F, t1803: F, t515: F, t996: F, t1504: F, t493: F, t1928: F, t435: F) -> (F, F, F, F, F) {
    let t8301 = t2957 * t8300;
    let t8303 = t2937 * t4893;
    let t8304 = t2957 * t8303;
    let t8306 = t1268 * t991;
    let t8308 = t1803 * t515;
    let t8309 = t996 * t8308;
    let t8310 = t493 * t1504;
    let t8311 = t8309 * t8310;
    let t8313 = t435 * t1928;
    (t8301, t8304, t8306, t8311, t8313)
}
