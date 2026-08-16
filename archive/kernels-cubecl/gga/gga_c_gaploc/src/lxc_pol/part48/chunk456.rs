//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 456/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk456<F: Float>(t203: F, t883: F, t900: F, t1359: F, t874: F, t1397: F, t2371: F, t1: F, t6540: F, t544: F, t1433: F, t2486: F) -> (F, F, F, F, F, F) {
    let t6589 = t883 * t203;
    let t6590 = t900 * t6589;
    let t6603 = t1359 * t874;
    let t6696 = t1397 * t2371;
    let t6699 = t6540 * t1;
    let t6700 = t544 * t6699;
    let t6710 = t1433 * t2486;
    (t6589, t6590, t6603, t6696, t6700, t6710)
}
