//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk827;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk828;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk829;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta167(t1548: f64, t775: f64, t800: f64, t4365: f64, t837: f64, t4364: f64, t125: f64, t1544: f64, t2747: f64, t1549: f64, t2703: f64, t124: f64, t4343: f64, t2749: f64, t2488: f64, t2653: f64, t2666: f64, t2678: f64, t2691: f64, t2695: f64, t2702: f64, t2716: f64, t2730: f64, t2739: f64, t2745: f64, t799: f64, t4439: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t4442, t4447, t4452, t4455, t4457) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk827(t1548, t775, t800, t4365, t837, t4364, t125, t1544, t2747, t1549, t2703, t124, t4343);
        let (t4458, t4462, t4468) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk828(t4457, t800, t2749, t4365, t2747, t2488, t2653, t2666, t2678, t2691, t2695, t2702, t2716, t2730, t2739, t2745, t4442, t4447, t4452, t4455, t799);
        let t4469 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk829(t4439, t4468);
    (t4442, t4447, t4452, t4455, t4458, t4462, t4469)
}
