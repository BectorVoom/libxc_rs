//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 469/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk469<F: Float>(t2172: F, t877: F, t881: F, t374: F, t880: F) -> (F, F, F, F) {
    let t2285 = 0.12361111111111111111e-1 * t2172;
    let t2291 = t877 * t881;
    let t2294 = t880 * t374;
    let t2295 = 1.0 / t2294;
    (t2285, t2291, t2294, t2295)
}
