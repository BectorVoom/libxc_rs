//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3387/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3387(t15393: f64, t15421: f64, t15397: f64, t52224: f64, t2918: f64, t2924: f64, t6110: f64, t11385: f64, t2875: f64, t6145: f64, t198: f64, t3336: f64, t336: f64, t63589: f64, t63592: f64, t63596: f64, t63600: f64, t63601: f64, t63607: f64, t63609: f64, t63612: f64, t63615: f64, t63618: f64) -> (f64, f64, f64, f64, f64) {
    let t63620 = 0.32163958997385070134e2_f64 * t15421 * t15393;
    let t63622 = 0.1034520258385468006e4_f64 * t52224 * t15397;
    let t63625 = 6.0_f64 * t2924 * t6110 * t2918;
    let t63628 = 0.57895126195293126241e3_f64 * t11385 * t6145 * t2875;
    let t63629 = -2.0_f64 * t198 * t3336 * t336 * t63601 + t63589 + t63592 + t63596 + t63600 - t63607 + t63609 + t63612 + t63615 - t63618 + t63620 + t63622 + t63625 + t63628;
    (t63620, t63622, t63625, t63628, t63629)
}
