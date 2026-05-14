//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 371/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk371<F: Float>(t653: F, t667: F, t1736: F, t1743: F, t1180: F, t676: F) -> (F, F, F, F) {
    let t1744 = t653 * t667;
    let t1745 = t1744 * t1736;
    let t1746 = t1743 * t1745;
    let t1749 = t676 * t1180;
    (t1744, t1745, t1746, t1749)
}
