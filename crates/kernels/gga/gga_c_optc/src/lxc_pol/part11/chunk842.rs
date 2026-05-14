//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 842/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk842<F: Float>(t16628: F, t7254: F, t914: F, t1415: F, t5059: F, t1235: F, t4565: F) -> (F, F, F, F) {
    let t16901 = t7254 * t16628;
    let t16902 = t914 * t16901;
    let t16912 = t5059 * t1415;
    let t16917 = t4565 * t1235;
    (t16901, t16902, t16912, t16917)
}
