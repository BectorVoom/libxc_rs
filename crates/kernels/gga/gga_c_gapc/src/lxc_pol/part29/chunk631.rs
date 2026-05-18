//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 631/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk631<F: Float>(t3738: F, t3739: F, t3725: F, t3730: F, t3735: F, t1096: F) -> (F, F) {
    let t3740 = t3738 * t3739;
    let t3742 = F::new(0.82073827867876094584e-5) * t3725 - F::new(0.11742981196020707897e-4) * t3730 - F::new(0.17098714139140853038e-6) * t3735 + F::new(0.73393632475129424356e-6) * t3740;
    let t3746 = t1096 * t1096;
    (t3742, t3746)
}
