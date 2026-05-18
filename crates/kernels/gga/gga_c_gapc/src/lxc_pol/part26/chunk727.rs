//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 727/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk727<F: Float>(t2936: F, t8578: F, t2948: F, t518: F, t1460: F, t2954: F, t1404: F, t2880: F, t120: F, t118: F, t1803: F, t61: F) -> (F, F, F, F, F) {
    let t8579 = t2936 * t8578;
    let t8581 = t518 * t2948;
    let t8583 = t1460 * t2954;
    let t8585 = t2880 * t1404;
    let t8586 = t120 * t8585;
    let t8588 = t1803 * t118;
    let t8589 = t61 * t8588;
    (t8579, t8581, t8583, t8586, t8589)
}
