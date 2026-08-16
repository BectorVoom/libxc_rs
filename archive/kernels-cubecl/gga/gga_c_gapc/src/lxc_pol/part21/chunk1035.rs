//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1035/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1035<F: Float>(t4939: F, t8676: F, t19765: F, t3141: F, t20500: F, t3712: F, t1: F, t424: F, t1038: F, t20594: F, t19586: F, t9260: F) -> (F, F, F, F, F, F) {
    let t25842 = t8676 * t4939;
    let t25871 = t3141 * t19765;
    let t25876 = t3712 * t20500;
    let t25953 = t424 * t1;
    let t26007 = t3712 * t1038 * t20594;
    let t26017 = t9260 * t1038 * t19586;
    (t25842, t25871, t25876, t25953, t26007, t26017)
}
