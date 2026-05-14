//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 615/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk615<F: Float>(t375: F, t5054: F, t89: F, t4934: F, t7514: F, t2336: F, t4930: F, t4926: F, t4918: F, t9725: F, t1882: F, t5161: F, t5157: F, t17720: F, t18145: F, t1775: F, t5102: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18148 = t89 * t375 * t5054;
    let t18159 = t7514 * t4934;
    let t18168 = t89 * t2336 * t4930;
    let t18171 = t89 * t2336 * t4926;
    let t18174 = t89 * t9725 * t4918;
    let t18188 = t1882 * t5161;
    let t18190 = t1882 * t5157;
    let t18241 = 2.0 / 9.0 * t17720;
    let t18265 = 2.0 / 3.0 * t18145;
    let t18266 = t18148 / 3.0;
    let t18283 = t1775 * t5102;
    (t18148, t18159, t18168, t18171, t18174, t18188, t18190, t18241, t18265, t18266, t18283)
}
