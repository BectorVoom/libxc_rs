//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 671/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk671<F: Float>(t1771: F, t745: F, t241: F, t9567: F, t1775: F, t2503: F, t2489: F, t2508: F, t458: F, t9698: F, t259: F, t89: F, t9555: F) -> (F, F, F, F, F, F, F) {
    let t9936 = t1771 * t745;
    let t9952 = t9567 * t241;
    let t9958 = t1775 * t2503;
    let t9960 = t1775 * t2489;
    let t9962 = t458 * t2508;
    let t9972 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t9698;
    let t9982 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t89 * t9555 * t259;
    (t9936, t9952, t9958, t9960, t9962, t9972, t9982)
}
