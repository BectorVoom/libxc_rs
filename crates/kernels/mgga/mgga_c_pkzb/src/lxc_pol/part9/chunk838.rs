//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 838/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk838<F: Float>(t179: F, t6405: F, t6406: F, t2370: F, t824: F, t2434: F, t2381: F, t2029: F, t919: F, t2387: F, t406: F, t931: F, t2185: F, t2888: F, t1485: F, t178: F, t405: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6408 = t179 * t6405 * t6406;
    let t6411 = t2370 * t824;
    let t6412 = t2434 * t6411;
    let t6413 = t2381 * t6412;
    let t6416 = t919 * t2029;
    let t6417 = t2370 * t2387;
    let t6418 = t6416 * t6417;
    let t6419 = t406 * t6418;
    let t6422 = t931 * t824;
    let t6423 = t6422 * t2185;
    let t6424 = t2888 * t6423;
    let t6428 = t178 * t1485 * t405;
    (t6408, t6412, t6413, t6416, t6417, t6418, t6419, t6423, t6424, t6428)
}
