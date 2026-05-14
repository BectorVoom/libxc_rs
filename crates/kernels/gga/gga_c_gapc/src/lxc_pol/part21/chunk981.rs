//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 981/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk981<F: Float>(t1084: F, t33895: F, t9929: F, t11910: F, t30095: F, t2562: F, t7120: F, t2560: F, t2568: F, t11953: F, t871: F, t2981: F, t787: F, t3752: F, t869: F, t11804: F, t11814: F, t19210: F) -> (F, F, F, F, F, F, F, F) {
    let t33897 = t1084 * t33895 * t9929;
    let t33899 = t11910 * t30095;
    let t33901 = t7120 * t2562;
    let t33902 = t2560 * t33901;
    let t33904 = t2568 * t33901;
    let t33906 = t871 * t11953;
    let t33908 = t33906 * t2981 * t787;
    let t33911 = t869 * t11953 * t3752;
    let t33914 = t11814 * t11804 * t19210;
    (t33897, t33899, t33902, t33904, t33906, t33908, t33911, t33914)
}
