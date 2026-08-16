//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 969/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk969(t30811: f64, t4273: f64, t129: f64, t507: f64, t7585: f64, t7587: f64, t30546: f64, t8477: f64, t1967: f64, t8561: f64, t30543: f64, t8515: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34340 = t30811 * t4273;
    let t34341 = 0.68598428988911579156e-2_f64 * t34340;
    let t34345 = t129 * t507;
    let t34347 = t7585 * t34345 * t7587;
    let t34348 = 0.14291339372689912324e-3_f64 * t34347;
    let t34349 = t30546 * t8477;
    let t34351 = t1967 * t8561;
    let t34352 = 0.37737710747524982482e-2_f64 * t34351;
    let t34361 = t30543 * t8515;
    (t34341, t34345, t34348, t34349, t34352, t34361)
}
