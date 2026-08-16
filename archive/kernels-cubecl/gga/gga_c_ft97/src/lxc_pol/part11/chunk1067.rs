//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1067/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1067<F: Float>(t1775: F, t9949: F, t2499: F, t8282: F, t2344: F, t2371: F, t2: F, t9931: F, t9917: F, t9897: F, t665: F, t7514: F) -> (F, F, F, F, F, F, F, F) {
    let t42105 = t1775 * t9949;
    let t42107 = t8282 * t2499;
    let t42109 = t2344 * t2371;
    let t42110 = t42109 * t2;
    let t42117 = t1775 * t9931;
    let t42119 = t1775 * t9917;
    let t42121 = t1775 * t9897;
    let t42123 = t665 * t7514;
    (t42105, t42107, t42109, t42110, t42117, t42119, t42121, t42123)
}
