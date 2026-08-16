//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1044/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1044(t30347: f64, t30811: f64, t4273: f64, t2068: f64, t7727: f64, t8480: f64, t129: f64, t507: f64, t7585: f64, t7587: f64, t30546: f64, t8477: f64) -> (f64, f64, f64, f64, f64) {
    let t34339 = 0.42874018118069736972e-3_f64 * t30347;
    let t34340 = t30811 * t4273;
    let t34341 = 0.68598428988911579156e-2_f64 * t34340;
    let t34343 = t2068 * t8480 * t7727;
    let t34345 = t129 * t507;
    let t34347 = t7585 * t34345 * t7587;
    let t34348 = 0.14291339372689912324e-3_f64 * t34347;
    let t34349 = t30546 * t8477;
    (t34339, t34341, t34343, t34348, t34349)
}
