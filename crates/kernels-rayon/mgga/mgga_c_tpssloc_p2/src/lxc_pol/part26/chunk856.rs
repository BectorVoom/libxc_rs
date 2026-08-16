//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 856/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk856(t10006: f64, t10044: f64, t2623: f64, t2643: f64, t2707: f64, t4178: f64, t831: f64, t843: f64, t9602: f64, t9604: f64, t9609: f64, t9613: f64, t9618: f64, t9623: f64, t9629: f64, t9634: f64, t9639: f64, t9963: f64) -> f64 {
    let t10046 = -t2623 * t2707 / 256.0_f64 - 119.0_f64 / 1152.0_f64 * t9602 + 7.0_f64 / 384.0_f64 * t9604 - 5.0_f64 / 128.0_f64 * t843 * t9609 - t9613 * t831 / 1024.0_f64 + 5.0_f64 / 256.0_f64 * t843 * t9618 - t2643 * t9623 / 1024.0_f64 - t4178 * t9629 / 128.0_f64 + t4178 * t9634 / 512.0_f64 - 7.0_f64 / 192.0_f64 * t9639 + t9963 + t10006 + t10044;
    t10046
}
