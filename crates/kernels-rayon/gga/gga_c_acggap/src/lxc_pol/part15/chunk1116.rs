//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1116/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1116(t1988: f64, t9724: f64, t2001: f64, t5966: f64, t1851: f64, t7605: f64, t5546: f64, t1761: f64, t30540: f64, t2095: f64, t39271: f64, t31491: f64, t39120: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39277 = t1988 * t9724;
    let t39279 = t2001 * t5966;
    let t39281 = t7605 * t1851;
    let t39283 = t2001 * t5546;
    let t39285 = t30540 * t1761;
    let t39292 = t2095 * t39271;
    let t39294 = t31491 * t39120;
    (t39277, t39279, t39281, t39283, t39285, t39292, t39294)
}
