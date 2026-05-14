//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 552/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk552<F: Float>(t4599: F, t5: F, t629: F, t4595: F, t1271: F) -> (F, F, F, F) {
    let t4615 = t5 * t4599;
    let t4616 = t629 * t4615;
    let t4619 = t5 * t4595;
    let t4620 = t629 * t4619;
    let t4623 = t1271 * t1271;
    (t4615, t4616, t4620, t4623)
}
