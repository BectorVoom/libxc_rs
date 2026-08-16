//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 588/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk588(t2268: f64, t3355: f64, t2798: f64, t921: f64, t1016: f64, t2355: f64) -> (f64, f64, f64, f64) {
    let t3357 = 0.56910013271352299198e-1_f64 * t2268 * t3355;
    let t3364 = t2798 * t921;
    let t3365 = t2355 * t1016;
    let t3366 = t1016 * t921;
    (t3357, t3364, t3365, t3366)
}
