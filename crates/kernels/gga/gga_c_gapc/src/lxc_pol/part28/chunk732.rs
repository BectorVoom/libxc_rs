//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 732/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk732<F: Float>(t1437: F, t8416: F, t2902: F, t424: F, t2915: F, t116: F, t1474: F, t152: F, t188: F, t505: F, t1947: F, t473: F) -> (F, F, F, F, F, F) {
    let t8417 = t1437 * t8416;
    let t8419 = t2902 * t424;
    let t8420 = t8419 * t2915;
    let t8422 = t1474 * t116;
    let t8423 = t8422 * t2915;
    let t8426 = t188 * t505 * t152;
    let t8427 = t8426 * t1947;
    let t8428 = t473 * t8427;
    (t8417, t8419, t8420, t8422, t8423, t8428)
}
