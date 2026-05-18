//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 835/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk835<F: Float>(t16989: F, t2221: F, t4828: F, t9099: F, t3565: F, t920: F, t2211: F, t2210: F, t1053: F, t18: F, t4431: F, t558: F) -> (F, F, F, F, F) {
    let t16990 = t2221 * t16989;
    let t16993 = t9099 * t4828;
    let t16996 = t920 * t3565;
    let t16997 = t2211 * t16996;
    let t16998 = t2210 * t16997;
    let t17001 = t18 * t1053;
    let t17002 = t2211 * t17001;
    let t17003 = t2210 * t17002;
    let t17006 = t4431 * t558;
    (t16990, t16993, t16998, t17003, t17006)
}
