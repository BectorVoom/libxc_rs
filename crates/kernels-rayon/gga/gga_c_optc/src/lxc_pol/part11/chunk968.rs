//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 968/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk968(t17744: f64, t389: f64, t17363: f64, t17425: f64, t17447: f64, t17453: f64, t17456: f64, t17460: f64, t17471: f64, t17504: f64, t17531: f64, t17733: f64) -> (f64, f64) {
    let t17746 = 0.62182e-1_f64 * t17744 * t389;
    let t17747 = -t17531 + t17456 - t17733 - t17447 + t17460 - t17746 - t17453 + t17471 - t17504 + t17363 + t17425;
    (t17746, t17747)
}
