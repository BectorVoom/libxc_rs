//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2024;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2025;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta470(t10868: f64, t241: f64, t820: f64, t14547: f64, t4364: f64, t4365: f64, t2724: f64, t2747: f64, t4450: f64, t14676: f64, t4366: f64, t10811: f64, t4452: f64, t2754: f64, t231: f64, t2394: f64, t10770: f64, t2719: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14894, t14896, t14900, t14904, t14907) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2024(t10868, t241, t820, t14547, t4364, t4365, t2724, t2747, t4450, t14676, t4366, t10811, t4452);
        let (t14910, t14914, t14917) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2025(t2747, t2754, t4450, t4364, t4365, t231, t2394);
        let (t14919, t14923) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2026(t10770, t14917, t4365, t2719, t820, t844);
    (t14894, t14896, t14900, t14904, t14907, t14910, t14914, t14917, t14919, t14923)
}
