//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 967/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk967(t30310: f64, t30314: f64, t30319: f64, t2304: f64, t7610: f64, t1988: f64, t8561: f64, t8566: f64, t1181: f64, t4521: f64, t604: f64, t7426: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34211 = 77.0_f64 / 864.0_f64 * t30310;
    let t34212 = 0.7640625e-2_f64 * t30314;
    let t34214 = 0.16006300097412701803e-1_f64 * t30319;
    let t34215 = t7610 * t2304;
    let t34217 = t1988 * t8561;
    let t34221 = t1988 * t8566;
    let t34237 = t7426 * t1181 * t604 * t4521;
    (t34211, t34212, t34214, t34215, t34217, t34221, t34237)
}
