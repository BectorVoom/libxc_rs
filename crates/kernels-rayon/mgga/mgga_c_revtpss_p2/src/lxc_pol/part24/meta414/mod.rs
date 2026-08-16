//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1357;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1358;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta414(t12046: f64, t15905: f64, t994: f64, t1014: f64, t11150: f64, t221: f64, t345: f64, t346: f64, t624: f64, t1065: f64, t215: f64, t373: f64, t675: f64, t828: f64, t11238: f64, t196: f64, t342: f64, t11626: f64, t358: f64, t3145: f64, t365: f64, t360: f64, t3153: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42690, t42731, t42745, t42778, t42792) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1357(t12046, t15905, t994, t1014, t11150, t221, t345, t346, t624, t1065, t215, t373, t675);
        let (t42793, t42859) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1358(t42792, t828, t11238, t196);
        let (t42860, t42862, t42865, t42866, t42868, t42871) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1359(t342, t42859, t11626, t358, t3145, t365, t360, t3153);
    (t42690, t42731, t42745, t42778, t42793, t42859, t42860, t42862, t42865, t42866, t42868, t42871)
}
