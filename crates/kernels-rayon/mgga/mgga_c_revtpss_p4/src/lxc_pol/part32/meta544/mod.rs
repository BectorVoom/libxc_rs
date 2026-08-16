//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1856;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta544(t25365: f64, t26506: f64, t25305: f64, t95540: f64, t10115: f64, t2063: f64, t10982: f64, t2061: f64, t9646: f64, t93190: f64, t95726: f64, t2435: f64, t26560: f64, t10073: f64, t2066: f64, t25390: f64, t886: f64, t7058: f64, t95730: f64, t2439: f64, t26434: f64, t887: f64, t2471: f64, t26563: f64, t10985: f64, t26576: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95888, t95891, t95893, t95899, t95902, t95905) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1856(t25365, t26506, t25305, t95540, t10115, t2063, t10982, t2061, t9646, t93190, t95726, t2435, t26560);
        let (t95911, t95914, t95925, t95927, t95930) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1857(t10073, t2066, t25390, t886, t7058, t95730, t2439, t26434, t887, t2471, t26563, t10985, t26576);
    (t95888, t95891, t95893, t95899, t95902, t95905, t95911, t95914, t95925, t95927, t95930)
}
