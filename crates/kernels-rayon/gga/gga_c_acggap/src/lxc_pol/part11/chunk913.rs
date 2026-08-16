//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 913/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk913(t30937: f64, t7566: f64, t1181: f64, t3730: f64, t604: f64, t7426: f64, t1170: f64, t1171: f64, t30538: f64, t1177: f64, t3529: f64, t4680: f64, t7569: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30938 = t30937 * t7566;
    let t30945 = t7426 * t1181 * t604 * t3730;
    let t30948 = t1170 * t30538 * t1171;
    let t30949 = t30948 * t1177;
    let t30956 = t7426 * t1181 * t604 * t3529;
    let t30963 = t7426 * t4680 * t7569;
    (t30938, t30945, t30948, t30949, t30956, t30963)
}
