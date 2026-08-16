//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 577/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk577(t4714: f64, t526: f64, t27: f64, t89: f64, t1957: f64, t3530: f64, t3535: f64, t4654: f64, t4658: f64, t4662: f64, t4666: f64, t4671: f64) -> (f64, f64, f64) {
    let t4715 = t526 * t4714;
    let t4717 = t89 * t27 * t4715;
    let t4719 = t1957 + t3530 + t3535 - t4654 / 27.0_f64 + t4658 / 9.0_f64 + t4662 / 9.0_f64 - t4666 / 18.0_f64 + t4671 / 3.0_f64 - t4717 / 6.0_f64;
    (t4715, t4717, t4719)
}
