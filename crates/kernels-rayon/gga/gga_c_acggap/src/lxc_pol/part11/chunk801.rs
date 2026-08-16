//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 801/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk801(t2274: f64, t7315: f64, t2016: f64, t2278: f64, t500: f64, t7329: f64, t1462: f64, t2001: f64, t1089: f64, t2080: f64, t535: f64, t2079: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8680 = t7315 * t2274;
    let t8682 = t2016 * t2278;
    let t8684 = t7329 * t500;
    let t8686 = t2001 * t1462;
    let t8689 = t1089 * t535 * t2080;
    let t8690 = t2079 * t8689;
    (t8680, t8682, t8684, t8686, t8689, t8690)
}
