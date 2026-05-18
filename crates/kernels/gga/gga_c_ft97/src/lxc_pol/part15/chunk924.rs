//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 924/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk924<F: Float>(t2399: F, t5134: F, t89: F, t38953: F, t5172: F, t5176: F, t8232: F, t2567: F, t5132: F, t737: F, t5167: F, t52212: F) -> (F, F, F, F, F, F, F) {
    let t68200 = t89 * t2399 * t5134;
    let t68220 = t38953 * t5172;
    let t68429 = t8232 * t5176;
    let t68528 = t5132 * t2567;
    let t68626 = t737 * t5132;
    let t68662 = t38953 * t5167;
    let t68751 = F::new(56.0) / F::new(243.0) * t52212;
    (t68200, t68220, t68429, t68528, t68626, t68662, t68751)
}
