//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 787/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk787<F: Float>(t1175: F, t4969: F, t724: F, t1168: F, t5064: F, t10052: F, t242: F, t10157: F, t21416: F, t265: F, t3977: F, t5147: F) -> (F, F, F, F, F, F) {
    let t21524 = t724 * t1175 * t4969;
    let t21531 = t5064 * t1168;
    let t21532 = t10052 * t21531;
    let t21533 = t242 * t21532;
    let t21537 = t10157 * t265 * t21416;
    let t21540 = t3977 * t5147;
    (t21524, t21531, t21532, t21533, t21537, t21540)
}
