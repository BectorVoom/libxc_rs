//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1029/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1029<F: Float>(t1512: F, t8274: F, t1504: F, t6340: F, t6382: F, t8267: F, t4235: F, t1513: F, t8260: F, t2267: F, t6310: F, t27235: F, t6322: F, t4230: F, t1492: F, t8240: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27411 = t1512 * t8274;
    let t27412 = t1504 * t27411;
    let t27414 = t6382 * t6340;
    let t27416 = t1512 * t8267;
    let t27417 = t4235 * t27416;
    let t27419 = t8260 * t1513;
    let t27421 = t6310 * t2267;
    let t27423 = t6322 * t27235;
    let t27424 = t4230 * t27423;
    let t27426 = t1492 * t8240;
    (t27411, t27412, t27414, t27416, t27417, t27419, t27421, t27423, t27424, t27426)
}
