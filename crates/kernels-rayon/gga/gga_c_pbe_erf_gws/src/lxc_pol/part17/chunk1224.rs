//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1224/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1224(t14733: f64, t51588: f64, t14469: f64, t51581: f64, t14423: f64, t343: f64, t361: f64, t50998: f64, t9505: f64, t14673: f64, t2397: f64, t3165: f64, t376: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52910 = t14733 * t51588;
    let t52912 = t51581 * t14469;
    let t52915 = t361 * t14423 * t343;
    let t52917 = t50998 * t52915 * t9505;
    let t52919 = t14673 * t2397;
    let t52921 = t376 * t3165;
    (t52910, t52912, t52915, t52917, t52919, t52921)
}
