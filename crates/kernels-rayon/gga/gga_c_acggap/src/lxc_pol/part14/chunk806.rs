//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 806/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk806(t157: f64, t406: f64, t556: f64, t7932: f64, t309: f64, t525: f64, t7963: f64, t609: f64, t939: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9025 = t556 * t406 * t157;
    let t9026 = t7932 * t9025;
    let t9029 = t525 * t309;
    let t9030 = t7932 * t9029;
    let t9031 = t7963 * t9030;
    let t9033 = t939 * t609;
    (t9025, t9026, t9029, t9030, t9031, t9033)
}
