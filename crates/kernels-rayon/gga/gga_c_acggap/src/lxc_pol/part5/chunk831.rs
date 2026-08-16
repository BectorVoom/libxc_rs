//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 831/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk831(t2667: f64, t2714: f64, t721: f64, t2707: f64, t772: f64, t203: f64, t281: f64, t84: f64, t985: f64, t132: f64, t2800: f64, t2804: f64) -> (f64, f64, f64, f64) {
    let t11560 = 0.4274e0_f64 * t721 * t2714 * t2667;
    let t11566 = 0.14246666666666666666e0_f64 * t721 * t2707 * t772;
    let t11570 = 0.18989649058080861537e-2_f64 * t281 * t203 * t985 * t84;
    let t11574 = 0.3684616320282908548e2_f64 * t721 * t132 * t2800 * t2804;
    (t11560, t11566, t11570, t11574)
}
