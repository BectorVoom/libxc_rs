//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 559/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk559<F: Float>(t3244: F, t390: F, t174: F, t943: F, t6: F, t965: F) -> (F, F, F) {
    let t3246 = F::cast_from(0.64311027177104605458e-3_f64) * t3244 * t390;
    let t3253 = t174 * t943;
    let t3266 = t6 * t965;
    (t3246, t3253, t3266)
}
