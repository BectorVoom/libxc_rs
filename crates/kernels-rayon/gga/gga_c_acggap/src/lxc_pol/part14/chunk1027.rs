//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1027/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1027(t36131: f64, t1988: f64, t8502: f64, t7799: f64, t8506: f64, t2290: f64, t7780: f64, t1423: f64, t7746: f64, t1507: f64, t2020: f64, t30120: f64, t8793: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36132 = 0.42874018118069736972e-3_f64 * t36131;
    let t36133 = t1988 * t8502;
    let t36134 = 0.42874018118069736972e-3_f64 * t36133;
    let t36135 = t7799 * t8506;
    let t36137 = t7780 * t2290;
    let t36139 = t7746 * t1423;
    let t36151 = t2020 * t1507;
    let t36152 = 7.0_f64 / 144.0_f64 * t36151;
    let t36156 = t30120 * t8793;
    (t36132, t36134, t36135, t36137, t36139, t36152, t36156)
}
