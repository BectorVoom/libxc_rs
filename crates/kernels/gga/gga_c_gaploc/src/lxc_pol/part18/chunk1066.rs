//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1066/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1066<F: Float>(t2021: F, t7530: F, t1854: F, t20901: F, t5679: F, t6110: F, t5580: F, t7426: F, t1: F, t23092: F, t22044: F, t739: F) -> (F, F, F, F, F, F) {
    let t23309 = t2021 * t7530;
    let t23310 = t20901 * t1854;
    let t23335 = t5679 * t6110;
    let t23344 = t5580 * t7426;
    let t23348 = t23092 * t1;
    let t23362 = t739 * t22044;
    (t23309, t23310, t23335, t23344, t23348, t23362)
}
