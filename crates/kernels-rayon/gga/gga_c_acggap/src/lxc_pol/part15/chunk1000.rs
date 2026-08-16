//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1000/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1000(t30861: f64, t8649: f64, t4372: f64, t7647: f64, t1427: f64, t1983: f64, t34186: f64, t7586: f64, t1545: f64, t30948: f64, t1456: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35240 = t30861 * t8649;
    let t35244 = t7647 * t4372;
    let t35246 = t1983 * t1427;
    let t35248 = t34186 * t7586 * t35246;
    let t35250 = t30948 * t1545;
    let t35258 = t7614 * t1456;
    (t35240, t35244, t35246, t35248, t35250, t35258)
}
