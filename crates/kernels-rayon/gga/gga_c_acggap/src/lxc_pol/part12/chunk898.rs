//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 898/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk898(t145: f64, t301: f64, t30598: f64, t721: f64, t174: f64, t372: f64, t7859: f64, t2016: f64, t7596: f64, t7343: f64, t7433: f64, t30105: f64, t7348: f64) -> (f64, f64, f64, f64, f64) {
    let t30601 = t30598 * t145 * t301 * t721;
    let t30605 = t7859 * t174 * t372 * t721;
    let t30607 = t2016 * t7596;
    let t30611 = t7433 * t7343;
    let t30613 = t30105 * t7348;
    (t30601, t30605, t30607, t30611, t30613)
}
