//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 637/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk637(t3873: f64, t576: f64, t3725: f64, t3730: f64, t3735: f64, t3740: f64, t1125: f64) -> (f64, f64, f64) {
    let t3874 = t576 * t3873;
    let t3879 = 0.32829531147150437834e-4_f64 * t3725 - 0.46971924784082831588e-4_f64 * t3730 - 0.68394856556563412154e-6_f64 * t3735 + 0.29357452990051769742e-5_f64 * t3740;
    let t3883 = t1125 * t1125;
    (t3874, t3879, t3883)
}
