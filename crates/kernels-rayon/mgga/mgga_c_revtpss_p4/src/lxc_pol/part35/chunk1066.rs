//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1066/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1066(t2106: f64, t9593: f64, t198: f64, t205: f64, t2070: f64, t72: f64, t8006: f64, t686: f64, t25375: f64, t25387: f64, t27216: f64, t7407: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28286 = t2106 * t9593;
    let t28291 = t198 * t205 * t2070;
    let t28313 = t8006 * t72;
    let t28314 = t28313 * t686;
    let t28315 = t25375 * t28314;
    let t28317 = t25387 * t28314;
    let t28352 = t27216 * t7407;
    (t28286, t28291, t28313, t28314, t28315, t28317, t28352)
}
