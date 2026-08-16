//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 973/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk973(t30543: f64, t8515: f64, t30398: f64, t30416: f64, t10146: f64, t420: f64, t576: f64, t1083: f64, t137: f64, t30444: f64, t1511: f64, t2020: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34361 = t30543 * t8515;
    let t34364 = 35.0_f64 / 216.0_f64 * t30398;
    let t34366 = 0.25158473831683321654e-2_f64 * t30416;
    let t34368 = t576 * t420 * t10146;
    let t34369 = t1083 * t137;
    let t34373 = 0.15724046144802076034e-2_f64 * t30444;
    let t34382 = t2020 * t1511;
    (t34361, t34364, t34366, t34368, t34369, t34373, t34382)
}
