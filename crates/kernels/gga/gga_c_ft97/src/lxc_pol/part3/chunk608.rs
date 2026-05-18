//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 608/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk608<F: Float>(t265: F, t4973: F, t724: F, t2594: F, t4965: F, t1154: F, t2475: F, t91: F, t2487: F, t4917: F, t2486: F, t2493: F, t4922: F) -> (F, F, F, F, F, F, F) {
    let t5083 = t724 * t265 * t4973;
    let t5087 = t2594 * t265 * t4965;
    let t5092 = t1154 * t1154;
    let t5094 = t91 * t2475 * t5092;
    let t5098 = t2487 * t4917;
    let t5099 = t2486 * t5098;
    let t5102 = t2493 * t4922;
    (t5083, t5087, t5092, t5094, t5098, t5099, t5102)
}
