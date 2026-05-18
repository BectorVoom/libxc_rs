//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 755/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk755<F: Float>(t7254: F, t7257: F, t914: F, t176: F, t2317: F, t998: F) -> (F, F, F, F) {
    let t7258 = t7254 * t7257;
    let t7259 = t914 * t7258;
    let t7262 = t176 * t2317;
    let t7263 = t7262 * t998;
    (t7258, t7259, t7262, t7263)
}
