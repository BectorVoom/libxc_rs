//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1149/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1149(t1882: f64, t34936: f64, t34918: f64, t358: f64, t1969: f64, t363: f64, t446: f64, t34924: f64, t34844: f64, t376: f64, t5890: f64, t148517: f64, t2112: f64, t28: f64) -> (f64, f64, f64, f64, f64) {
    let t148573 = t1882 * t34936;
    let t148575 = t34918 * t358;
    let t148578 = t446 * t1969 * t148575 * t363;
    let t148580 = t1882 * t34924;
    let t148583 = t5890 * t376 * t34844;
    let t148587 = t5890 * t28 * t2112 * t148517;
    (t148573, t148578, t148580, t148583, t148587)
}
