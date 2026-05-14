//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 504/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk504<F: Float>(t2725: F, t5284: F, t2730: F, t3796: F, t3804: F, t5031: F, t5034: F, t5039: F, t5043: F, t5047: F) -> (F, F) {
    let t5285 = t2725 * t5284;
    let t5295 = 0.48897200801234567903e0 * t5031 - 0.88904001456790123461e-1 * t3796 - 0.88904001456790123461e-1 * t5034 - t2730 + 0.11113000182098765433e-1 * t3804 + 0.22226000364197530865e-1 * t5039 - 0.33339000546296296298e-1 * t5043 + 0.16669500273148148149e-1 * t5047;
    (t5285, t5295)
}
