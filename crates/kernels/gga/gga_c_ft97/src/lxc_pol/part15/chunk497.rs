//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 497/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk497<F: Float>(t1217: F, t5206: F, t2660: F, t4917: F, t2345: F, t89: F, t1091: F, t1212: F, t2665: F, t446: F, t2670: F, t666: F, t4635: F, t792: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5207 = t5206 * t1217;
    let t5209 = t2660 * t4917;
    let t5211 = t89 * t2345 * t5209;
    let t5213 = t1091 * t1212;
    let t5214 = t2665 * t5213;
    let t5215 = t446 * t5214;
    let t5217 = t2670 * t4917;
    let t5219 = t89 * t666 * t5217;
    let t5221 = t792 * t4635;
    let t5223 = t89 * t666 * t5221;
    let t5225 = t1212 * t1212;
    (t5207, t5209, t5211, t5213, t5214, t5215, t5217, t5219, t5221, t5223, t5225)
}
