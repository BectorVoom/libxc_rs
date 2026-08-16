//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1212/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1212(t1844: f64, t309: f64, t7932: f64, t7963: f64, t39499: f64, t7942: f64, t463: f64, t1960: f64, t5517: f64, t157: f64, t1658: f64, t524: f64) -> (f64, f64, f64, f64, f64) {
    let t40733 = t1844 * t309;
    let t40735 = t7963 * t7932 * t40733;
    let t40738 = t7942 * t7932 * t39499;
    let t40740 = t1844 * t463;
    let t40746 = t1960 * t5517;
    let t40749 = t1658 * t524 * t157;
    (t40735, t40738, t40740, t40746, t40749)
}
