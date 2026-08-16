//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1129/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1129<F: Float>(t16534: F, t7122: F, t16382: F, t7110: F, t16474: F, t23077: F, t16483: F, t7037: F, t16402: F, t16543: F, t9917: F, t16540: F) -> (F, F, F, F, F, F, F) {
    let t48866 = t7122 * t16534;
    let t48875 = t7110 * t16382;
    let t48904 = t23077 * t16474;
    let t48906 = t7037 * t16483;
    let t48922 = t7110 * t16402;
    let t48924 = t9917 * t16543;
    let t48960 = t7122 * t16540;
    (t48866, t48875, t48904, t48906, t48922, t48924, t48960)
}
