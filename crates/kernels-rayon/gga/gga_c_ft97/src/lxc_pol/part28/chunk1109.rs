//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1109/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1109(t1008: f64, t32318: f64, t32852: f64, t1013: f64, t139046: f64, t2035: f64, t3404: f64, t7318: f64, t32791: f64, t1701: f64, t26739: f64, t26750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t147412 = t32852 * t32318 * t1008;
    let t147416 = t139046 * t32318 * t1013;
    let t147425 = t2035 * t7318 * t3404;
    let t147429 = t2035 * t32791 * t1008;
    let t147432 = t1701 * t26739;
    let t147435 = t1701 * t26750;
    (t147412, t147416, t147425, t147429, t147432, t147435)
}
