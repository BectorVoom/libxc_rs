//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1391;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1392;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta363(t14495: f64, t2797: f64, t2782: f64, t1558: f64, t860: f64, t231: f64, t2783: f64, t251: f64, t4423: f64, t10073: f64, t4496: f64, t10542: f64, t4500: f64, t4424: f64, t72: f64, t686: f64, t2798: f64, t136: f64, t1559: f64, t2457: f64, t10535: f64, t10069: f64, t1568: f64, t836: f64, t10867: f64, t225: f64, t213: f64, t2777: f64, t4518: f64, t2439: f64, t2470: f64, t4499: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14498, t14506, t14511, t14512, t14518) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1391(t14495, t2797, t2782, t1558, t860, t231, t2783, t251, t4423, t10073, t4496, t10542, t4500);
        let (t14522, t14525, t14533, t14535) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1392(t4424, t72, t686, t2798, t136, t1559, t2457, t10535, t10069, t4496, t1568, t836);
        let (t14539, t14546, t14558, t14563) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1393(t14535, t231, t2783, t2782, t10867, t225, t213, t2777, t4518, t2439, t2470, t4499);
    (t14498, t14506, t14511, t14512, t14518, t14522, t14525, t14533, t14539, t14546, t14558, t14563)
}
