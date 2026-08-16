//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 845/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk845<F: Float>(t3103: F, t885: F, t3379: F, t2520: F, t2972: F, t3384: F, t787: F, t7927: F, t3396: F, t325: F, t8992: F, t2817: F) -> (F, F, F, F) {
    let t9658 = t885 * t3103;
    let t9659 = t9658 * t3379;
    let t9661 = t2520 * t2972;
    let t9662 = t9661 * t3384;
    let t9664 = t7927 * t787;
    let t9665 = t3396 * t9664;
    let t9667 = t325 * t8992;
    let t9668 = t9667 * t2817;
    (t9659, t9662, t9665, t9668)
}
