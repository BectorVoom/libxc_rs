//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 763/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk763<F: Float>(t2660: F, t7880: F, t8655: F, t3434: F, t969: F, t1936: F, t928: F, t943: F, t3056: F, t19: F, t932: F, t3114: F, t1087: F, t3406: F, t829: F, t954: F) -> (F, F, F, F, F, F, F, F) {
    let t9775 = t2660 * t8655 * t7880;
    let t9777 = t3434 * t969;
    let t9779 = t928 * t1936;
    let t9780 = t9779 * t943;
    let t9782 = t928 * t3056;
    let t9783 = t9782 * t943;
    let t9785 = t932 * t19;
    let t9786 = t9785 * t3114;
    let t9787 = t1087 * t3406;
    let t9788 = t829 * t9787;
    let t9789 = t9786 * t9788;
    let t9791 = t3434 * t954;
    (t9775, t9777, t9780, t9783, t9786, t9787, t9789, t9791)
}
