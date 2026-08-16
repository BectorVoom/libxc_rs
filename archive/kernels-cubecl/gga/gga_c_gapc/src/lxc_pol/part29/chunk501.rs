//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 501/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk501<F: Float>(t116: F, t1457: F, t2920: F, t134: F, t190: F, t1954: F, t200: F, t1475: F, t996: F, t493: F, t568: F, t1004: F, t423: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2921 = t116 * t1457;
    let t2922 = t2920 * t2921;
    let t2923 = t190 * t134;
    let t2925 = t2923 * t200 * t1954;
    let t2926 = t2922 * t2925;
    let t2928 = t996 * t1475;
    let t2929 = t493 * t568;
    let t2930 = t2928 * t2929;
    let t2932 = t1004 * t423;
    (t2921, t2922, t2923, t2925, t2926, t2928, t2929, t2930, t2932)
}
