//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1603;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1604;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta440(t1882: f64, t1892: f64, t4003: f64, t5658: f64, t10032: f64, t10035: f64, t10044: f64, t1399: f64, t14116: f64, t14120: f64, t14126: f64, t14131: f64, t14146: f64, t14149: f64, t14158: f64, t14161: f64, t14166: f64, t4118: f64, t5735: f64, t5745: f64, t5755: f64, t6844: f64, t820: f64, t555: f64, t6861: f64, t6843: f64, t1398: f64, t9994: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t9934: f64, t3989: f64, t6856: f64, t13762: f64, t13763: f64, t13765: f64, t13772: f64, t13778: f64, t9711: f64, t9712: f64, t9725: f64, t9729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21981, t21990, t21998) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1603(t1882, t1892, t4003, t5658, t10032, t10035, t10044, t1399, t14116, t14120, t14126, t14131, t14146, t14149, t14158, t14161, t14166, t4118, t5735, t5745, t5755, t6844, t820);
        let (t22005, t22009, t22016, t22021, t22023, t22025, t22026, t22028, t22030) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1604(t555, t6861, t6843, t1398, t9994, t550, t543, t3992, t2661, t4003, t9934, t3989, t6856);
        let t22035 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1605(t13762, t13763, t13765, t13772, t13778, t22023, t22028, t22030, t9711, t9712, t9725, t9729);
    (t21981, t21990, t21998, t22005, t22009, t22016, t22021, t22025, t22026, t22035)
}
