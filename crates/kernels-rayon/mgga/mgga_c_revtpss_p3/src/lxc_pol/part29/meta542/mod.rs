//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1877;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta542(t25365: f64, t26544: f64, t93190: f64, t95726: f64, t2435: f64, t26560: f64, t10073: f64, t2066: f64, t25390: f64, t886: f64, t7058: f64, t95730: f64, t2439: f64, t26434: f64, t887: f64, t2471: f64, t26563: f64, t10985: f64, t26576: f64, t2062: f64, t2769: f64, t786: f64, t10997: f64, t26519: f64, t93157: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95900, t95902, t95905, t95911, t95914) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1877(t25365, t26544, t93190, t95726, t2435, t26560, t10073, t2066, t25390, t886, t7058, t95730);
        let (t95925, t95927, t95930, t95936, t95937, t95945) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1878(t2439, t26434, t887, t2471, t26563, t10985, t26576, t2062, t2769, t786, t10997, t26519, t93157);
    (t95900, t95902, t95905, t95911, t95914, t95925, t95927, t95930, t95936, t95937, t95945)
}
