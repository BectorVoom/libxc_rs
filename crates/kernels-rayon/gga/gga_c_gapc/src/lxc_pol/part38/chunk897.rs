//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 897/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk897(t10136: f64, t10170: f64, t10200: f64, t10240: f64, t10283: f64, t10323: f64, t10370: f64, t10405: f64, t3649: f64, t423: f64, t1459: f64, t3652: f64) -> (f64, f64, f64, f64) {
    let t10408 = t10136 + t10170 + t10200 + t10240 + t10283 + t10323 + t10370 + t10405;
    let t11181 = t3649 * t423;
    let t11182 = t11181 * t1459;
    let t11183 = t11182 * t3652;
    (t10408, t11181, t11182, t11183)
}
