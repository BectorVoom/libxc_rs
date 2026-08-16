//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 837/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk837(t1784: f64, t570: f64, t1886: f64, t2001: f64, t1881: f64, t1844: f64, t599: f64, t1181: f64, t2068: f64, t336: f64, t5630: f64, t8040: f64, t9476: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9751 = t570 * t1784;
    let t9753 = t2001 * t1886;
    let t9755 = t2001 * t1881;
    let t9757 = t599 * t1844;
    let t9758 = t1181 * t9757;
    let t9759 = t2068 * t9758;
    let t9761 = t336 * t5630;
    let t9762 = t570 * t9761;
    let t9826 = t8040 * t9476;
    (t9751, t9753, t9755, t9757, t9758, t9759, t9761, t9762, t9826)
}
