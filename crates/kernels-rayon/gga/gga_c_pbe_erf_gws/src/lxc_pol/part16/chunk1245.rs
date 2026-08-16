//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1245/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1245(t14456: f64, t51666: f64, t13796: f64, t14423: f64, t14637: f64, t6524: f64, t1118: f64, t13859: f64, t2195: f64, t13781: f64, t3222: f64, t3306: f64, t3972: f64, param_a_c: f64) -> (f64, f64, f64, f64) {
    let t53545 = t51666 * t14456;
    let t53549 = t14637 * t13796 * t14423 * t6524;
    let t53553 = t13859 * t13796 * t1118 * t2195;
    let t53562 = t3972 * t13781 * t3306 * param_a_c * t3222;
    (t53545, t53549, t53553, t53562)
}
