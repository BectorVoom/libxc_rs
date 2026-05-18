//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1041/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1041<F: Float>(t123: F, t1559: F, t160: F, t4348: F, t892: F, t2486: F, t4803: F, t594: F, t874: F, t1265: F, t1415: F, t6953: F) -> (F, F, F, F, F, F) {
    let t19531 = t1559 * t123;
    let t19532 = t19531 * t160;
    let t19933 = t892 * t4348;
    let t20003 = t4803 * t2486;
    let t20008 = t594 * t874;
    let t20013 = t874 * t1265;
    let t20018 = t1415 * t6953;
    (t19532, t19933, t20003, t20008, t20013, t20018)
}
