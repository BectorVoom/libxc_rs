//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 628/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk628(t3738: f64, t3739: f64, t3725: f64, t3730: f64, t3735: f64, t1096: f64) -> (f64, f64) {
    let t3740 = t3738 * t3739;
    let t3742 = 0.82073827867876094584e-5_f64 * t3725 - 0.11742981196020707897e-4_f64 * t3730 - 0.17098714139140853038e-6_f64 * t3735 + 0.73393632475129424356e-6_f64 * t3740;
    let t3746 = t1096 * t1096;
    (t3742, t3746)
}
