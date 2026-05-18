//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 612/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk612<F: Float>(t5147: F, t762: F, t242: F, t1168: F, t3977: F, t1131: F, t1175: F, t729: F, t265: F, t5053: F, t992: F, t2600: F) -> (F, F, F, F, F, F, F, F) {
    let t5148 = t762 * t5147;
    let t5149 = t242 * t5148;
    let t5152 = t3977 * t1168;
    let t5153 = t242 * t5152;
    let t5157 = t729 * t1175 * t1131;
    let t5161 = t729 * t265 * t5053;
    let t5165 = t992 * t1131;
    let t5166 = t2600 * t5165;
    (t5148, t5149, t5152, t5153, t5157, t5161, t5165, t5166)
}
