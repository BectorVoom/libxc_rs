//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1170/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1170(t2268: f64, t31585: f64, t426: f64, t535: f64, t1222: f64, t3344: f64, t10262: f64, t484: f64, t1217: f64, t3351: f64, t2317: f64, t6525: f64, t7901: f64) -> (f64, f64, f64, f64, f64) {
    let t31685 = 0.56910013271352299198e-1_f64 * t2268 * t535 * t31585 * t426;
    let t31687 = t1222 * t3344;
    let t31688 = 0.31616674039640166222e-2_f64 * t31687;
    let t31689 = t484 * t10262;
    let t31690 = 0.31616674039640166222e-2_f64 * t31689;
    let t31691 = t1217 * t3351;
    let t31692 = 0.36886119712913527259e-2_f64 * t31691;
    let t31694 = t6525 * t7901 * t2317;
    (t31685, t31688, t31690, t31692, t31694)
}
