//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 505/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk505(t1036: f64, t2974: f64, t2973: f64, t128: f64, t512: f64, t19: f64) -> (f64, f64, f64) {
    let t2975 = t1036 * t2974;
    let t2976 = t2973 * t2975;
    let t2978 = t128 * t512;
    let t2979 = t2978 * t19;
    (t2975, t2976, t2979)
}
