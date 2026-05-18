//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 649/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk649<F: Float>(t1339: F, t161: F, t1422: F, t1427: F, t1415: F, t129: F) -> (F, F, F, F) {
    let t4812 = t1339 * t161;
    let t4818 = t1427 * t1422;
    let t4819 = t1415 * t4818;
    let t4820 = t161 * t129;
    (t4812, t4818, t4819, t4820)
}
