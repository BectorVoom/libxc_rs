//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 667/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk667<F: Float>(t152: F, t3638: F, t5918: F, t434: F, t144: F, t467: F, t458: F, t1437: F, t2902: F, t424: F, t2915: F, t116: F, t1474: F, t188: F, t505: F, t1947: F) -> (F, F, F, F, F, F, F, F) {
    let t8411 = t3638 * t152;
    let t8412 = t8411 * t5918;
    let t8413 = t434 * t8412;
    let t8415 = t467 * t144;
    let t8416 = t8415 * t458;
    let t8417 = t1437 * t8416;
    let t8419 = t2902 * t424;
    let t8420 = t8419 * t2915;
    let t8422 = t1474 * t116;
    let t8423 = t8422 * t2915;
    let t8426 = t188 * t505 * t152;
    let t8427 = t8426 * t1947;
    (t8413, t8415, t8417, t8419, t8420, t8422, t8423, t8427)
}
