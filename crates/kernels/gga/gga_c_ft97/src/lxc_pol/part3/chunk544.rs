//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 544/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk544<F: Float>(t2348: F, t4917: F, t2345: F, t89: F, t1091: F, t1131: F, t2354: F, t446: F, t2361: F, t666: F, t4635: F, t669: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4918 = t2348 * t4917;
    let t4920 = t89 * t2345 * t4918;
    let t4922 = t1091 * t1131;
    let t4923 = t2354 * t4922;
    let t4924 = t446 * t4923;
    let t4926 = t2361 * t4917;
    let t4928 = t89 * t666 * t4926;
    let t4930 = t669 * t4635;
    let t4932 = t89 * t666 * t4930;
    let t4934 = t1131 * t1131;
    (t4918, t4920, t4922, t4923, t4924, t4926, t4928, t4930, t4932, t4934)
}
