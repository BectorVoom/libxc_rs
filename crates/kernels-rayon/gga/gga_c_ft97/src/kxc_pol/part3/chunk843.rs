//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 843/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk843(t4714: f64, t558: f64, t167: f64, t2185: f64, t4829: f64, t8392: f64, t1053: f64, t12277: f64, t144: f64, t3590: f64, t569: f64, t925: f64) -> (f64, f64, f64, f64, f64) {
    let t17099 = t4714 * t558;
    let t17101 = t2185 * t167 * t17099;
    let t17104 = t8392 * t4829;
    let t17106 = t12277 * t1053;
    let t17107 = t144 * t17106;
    let t17111 = t569 * t3590 * t925;
    (t17101, t17104, t17106, t17107, t17111)
}
