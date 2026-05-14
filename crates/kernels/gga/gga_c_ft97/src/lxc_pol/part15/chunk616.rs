//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 616/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk616<F: Float>(t1775: F, t5110: F, t2: F, t4934: F, t5099: F, t5106: F, t458: F, t5118: F, t5114: F, t5092: F, t9890: F, t18168: F, t18171: F, t18174: F, t5132: F, t761: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18286 = t1775 * t5110;
    let t18293 = t2 * t4934;
    let t18303 = t1775 * t5099;
    let t18305 = t1775 * t5106;
    let t18314 = t458 * t5118;
    let t18316 = t458 * t5114;
    let t18370 = t9890 * t5092;
    let t18381 = t18168 / 9.0;
    let t18382 = 2.0 / 9.0 * t18171;
    let t18383 = 2.0 / 27.0 * t18174;
    let t18391 = t5132 * t761;
    (t18286, t18293, t18303, t18305, t18314, t18316, t18370, t18381, t18382, t18383, t18391)
}
