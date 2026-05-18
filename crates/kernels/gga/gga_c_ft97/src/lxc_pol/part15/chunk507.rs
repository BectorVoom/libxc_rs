//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 507/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk507<F: Float>(t2487: F, t4917: F, t2486: F, t2493: F, t4922: F, t2497: F, t737: F, t4635: F, t738: F, t192: F, t2506: F, t4934: F) -> (F, F, F, F, F, F, F, F) {
    let t5098 = t2487 * t4917;
    let t5099 = t2486 * t5098;
    let t5102 = t2493 * t4922;
    let t5105 = t2497 * t4917;
    let t5106 = t737 * t5105;
    let t5109 = t738 * t4635;
    let t5110 = t737 * t5109;
    let t5114 = t192 * t2506 * t4934;
    (t5098, t5099, t5102, t5105, t5106, t5109, t5110, t5114)
}
