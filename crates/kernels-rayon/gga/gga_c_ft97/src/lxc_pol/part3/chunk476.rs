//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 476/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk476(t184: f64, t3596: f64, t1063: f64, t5: f64, t1068: f64, t2253: f64, t179: f64, t422: f64, t2984: f64, t2266: f64, t643: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3597 = t3596 * t184;
    let t3601 = t5 * t1063;
    let t3611 = t2253 * t1068;
    let t3613 = t422 * t179;
    let t3614 = t3613 * t2984;
    let t3618 = t2266 * t925 * t643;
    (t3597, t3601, t3611, t3613, t3614, t3618)
}
