//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 798/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk798<F: Float>(t127: F, t16370: F, t6: F, t161: F, t16324: F, t1271: F, t4649: F, t162: F, t1256: F, t13174: F, t2034: F, t3353: F, t3360: F, t4599: F, t6931: F, t13214: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16372 = t6 * t16370 * t127;
    let t16373 = t161 * t16372;
    let t16376 = t16324 * t127;
    let t16377 = t161 * t16376;
    let t16380 = t4649 * t1271;
    let t16381 = t16380 * t127;
    let t16382 = t162 * t16381;
    let t16385 = t13174 * t1256;
    let t16386 = t2034 * t16385;
    let t16389 = t3353 * t4649;
    let t16390 = t162 * t16389;
    let t16393 = t3360 * t4599;
    let t16394 = t6931 * t16393;
    let t16397 = t13214 * t1256;
    (t16372, t16373, t16376, t16377, t16380, t16381, t16382, t16385, t16386, t16389, t16390, t16393, t16394, t16397)
}
