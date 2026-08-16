//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3723/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3723(t12866: f64, t5406: f64, t58895: f64, t17789: f64, t21306: f64, t17401: f64, t17617: f64, t15687: f64, t17394: f64, t3782: f64, t1122: f64, t5284: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70612 = t12866 * t58895 * t5406;
    let t70616 = t21306 * t17789;
    let t70623 = t17401 * t17617;
    let t70629 = t17394 * t15687;
    let t70630 = t3782 * t70629;
    let t70633 = t1122 * t5284;
    (t70612, t70616, t70623, t70629, t70630, t70633)
}
