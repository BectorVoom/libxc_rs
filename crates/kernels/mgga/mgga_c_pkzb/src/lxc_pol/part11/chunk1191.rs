//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1191/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1191<F: Float>(t31456: F, t31458: F, t31461: F, t31464: F, t31653: F, t31950: F, t31951: F, t31953: F, t31957: F, t31960: F, t31962: F, t31965: F, t31967: F, t31986: F, t11371: F, t2099: F, t918: F) -> (F, F) {
    let t31989 = t31950 + t31951 + t31953 - t31653 + t31456 - t31458 - t31461 + t31464 - t31957 + t31960 + t31962 - t31965 - t31967 + t31986;
    let t31996 = t918 * t2099 * t11371;
    (t31989, t31996)
}
