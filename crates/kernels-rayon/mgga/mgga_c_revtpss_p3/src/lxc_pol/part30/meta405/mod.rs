//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1514;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1515;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta405(t14767: f64, t2477: f64, t828: f64, t1544: f64, t2394: f64, t10698: f64, t10811: f64, t4462: f64, t4416: f64, t808: f64, t10886: f64, t2703: f64, t4458: f64, t10678: f64, t10682: f64, t10687: f64, t10692: f64, t14759: f64, t14761: f64, t14765: f64, t851: f64, t10769: f64, t836: f64, t2749: f64, t2746: f64, t14494: f64, t775: f64, t14586: f64, t10693: f64, t10706: f64, t10711: f64, t10713: f64, t10717: f64, t10719: f64, t10723: f64, t10730: f64, t10734: f64, t10742: f64, t2745: f64, t4362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14769, t14772, t14774, t14777, t14780, t14783) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1514(t14767, t2477, t828, t1544, t2394, t10698, t10811, t4462, t4416, t808, t10886, t2703, t4458);
        let t14784 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1515(t10678, t10682, t10687, t10692, t14759, t14761, t14765, t14769, t14774, t14777, t14780, t14783, t851);
        let (t14788, t14793, t14804, t14811) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1516(t10769, t828, t1544, t836, t2749, t2746, t14494, t775, t14586, t10693, t10706, t10711, t10713, t10717, t10719, t10723, t10730, t10734, t10742, t2745, t4362);
    (t14769, t14772, t14774, t14784, t14788, t14793, t14804, t14811)
}
