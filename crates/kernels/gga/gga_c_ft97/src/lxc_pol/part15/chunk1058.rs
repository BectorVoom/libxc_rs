//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1058/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1058<F: Float>(t45662: F, t58730: F, t85463: F, t85467: F, t85472: F, t85476: F, t85481: F, t85485: F, t85489: F, t85493: F, t85498: F, t85504: F) -> F {
    let t86863 = F::cast_from(0.66678001092592592595e-1_f64) * t85463 + F::cast_from(0.8890400145679012346e-1_f64) * t85467 - F::cast_from(0.40006800655555555556e0_f64) * t85472 + F::cast_from(0.60010200983333333334e0_f64) * t85476 + F::cast_from(0.44452000728395061732e-1_f64) * t58730 - F::cast_from(0.62232801019753086422e0_f64) * t85481 + F::cast_from(0.31116400509876543211e0_f64) * t85485 + F::cast_from(0.80013601311111111114e0_f64) * t85489 - F::cast_from(0.80013601311111111114e0_f64) * t85493 + F::cast_from(0.2469555596021947874e-1_f64) * t45662 + F::cast_from(0.17286889172153635117e0_f64) * t85498 + F::cast_from(0.16669500273148148149e-1_f64) * t85504;
    t86863
}
