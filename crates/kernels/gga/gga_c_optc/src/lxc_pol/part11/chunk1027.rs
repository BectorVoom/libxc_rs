//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1027/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1027<F: Float>(t16474: F, t23077: F, t16483: F, t7037: F, t16402: F, t7110: F, t16543: F, t9917: F, t16540: F, t7122: F, t16537: F, t16546: F, t16406: F, t16505: F, t2120: F, t16487: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t48904 = t23077 * t16474;
    let t48906 = t7037 * t16483;
    let t48922 = t7110 * t16402;
    let t48924 = t9917 * t16543;
    let t48960 = t7122 * t16540;
    let t48962 = t7122 * t16537;
    let t48990 = t9917 * t16546;
    let t48992 = t7110 * t16406;
    let t49019 = t2120 * t16505;
    let t49023 = t2120 * t16487;
    (t48904, t48906, t48922, t48924, t48960, t48962, t48990, t48992, t49019, t49023)
}
