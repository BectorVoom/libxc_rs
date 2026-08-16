//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 849/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk849(t12680: f64, t3420: f64, t4805: f64, t604: f64, t379: f64, t2210: f64, t12724: f64, t16150: f64, t12723: f64, t16169: f64, t3440: f64, t3439: f64) -> (f64, f64, f64, f64) {
    let t17195 = t12680 * t3420;
    let t17198 = t604 * t4805;
    let t17199 = t17198 * t379;
    let t17200 = t2210 * t17199;
    let t17203 = t12724 * t16150;
    let t17204 = t12723 * t17203;
    let t17207 = t3440 * t16169;
    let t17208 = t3439 * t17207;
    (t17195, t17200, t17204, t17208)
}
