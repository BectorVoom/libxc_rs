//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 539/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk539<F: Float>(t2917: F, t2918: F, t4917: F, t1091: F, t1268: F, t2923: F, t231: F, t2928: F, t4635: F, t893: F) -> (F, F, F, F, F) {
    let t5442 = t2917 * t2918 * t4917;
    let t5446 = t2923 * t1091 * t1268;
    let t5450 = t231 * t2928 * t4917;
    let t5454 = t231 * t893 * t4635;
    let t5457 = t1268 * t1268;
    (t5442, t5446, t5450, t5454, t5457)
}
