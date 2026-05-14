//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 740/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk740<F: Float>(t2520: F, t2972: F, t3384: F, t787: F, t7927: F, t3396: F, t325: F, t8992: F, t2817: F, t3321: F, t3320: F, t1084: F, t8686: F, t1089: F, t5542: F, t7953: F) -> (F, F, F, F, F, F, F, F) {
    let t9661 = t2520 * t2972;
    let t9662 = t9661 * t3384;
    let t9664 = t7927 * t787;
    let t9665 = t3396 * t9664;
    let t9667 = t325 * t8992;
    let t9668 = t9667 * t2817;
    let t9670 = t7927 * t3321;
    let t9671 = t3320 * t9670;
    let t9673 = t1084 * t8686;
    let t9674 = t9673 * t1089;
    let t9677 = t7953 * t5542;
    (t9662, t9665, t9668, t9670, t9671, t9673, t9674, t9677)
}
