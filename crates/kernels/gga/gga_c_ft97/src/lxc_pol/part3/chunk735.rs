//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 735/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk735<F: Float>(t16996: F, t2211: F, t2210: F, t1053: F, t18: F, t4431: F, t558: F, t2222: F, t2221: F, t609: F, t2178: F, t4724: F, t379: F, t160: F, t4668: F, t9133: F) -> (F, F, F, F, F, F) {
    let t16997 = t2211 * t16996;
    let t16998 = t2210 * t16997;
    let t17001 = t18 * t1053;
    let t17002 = t2211 * t17001;
    let t17003 = t2210 * t17002;
    let t17006 = t4431 * t558;
    let t17007 = t2222 * t17006;
    let t17008 = t2221 * t17007;
    let t17011 = t4431 * t609;
    let t17012 = t2211 * t17011;
    let t17013 = t2210 * t17012;
    let t17016 = t2178 * t4724;
    let t17017 = t17016 * t379;
    let t17018 = t2210 * t17017;
    let t17021 = t160 * t4668;
    let t17022 = t17021 * t379;
    let t17023 = t9133 * t17022;
    (t16998, t17003, t17008, t17013, t17018, t17023)
}
