//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 930/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk930<F: Float>(t147: F, t24100: F, t24151: F, t184: F, t5: F, t5981: F, t1395: F, t1580: F, t21: F, t2301: F, t2306: F, t2309: F, t363: F, t5982: F, t5985: F, t650: F, t108: F, t3103: F) -> (F, F, F, F, F) {
    let t148 = 10000000.0 <= t147;
    let t24152 = t24100 + t24151;
    let t24153 = t24152 * t184;
    let t24157 = t5 * t5981;
    let t24173 = piecewise3(t148, 0.0, t5 * t24153 * t21 / 4.0 + t24157 * t650 / 2.0 + t5 * t5982 * t363 / 2.0 + t5985 * t2301 / 4.0 + t5985 * t2306 / 4.0 + t5985 * t2309 / 2.0 + t5 * t1395 * t1580 / 4.0);
    let t25523 = t108 * t3103;
    (t24152, t24153, t24157, t24173, t25523)
}
