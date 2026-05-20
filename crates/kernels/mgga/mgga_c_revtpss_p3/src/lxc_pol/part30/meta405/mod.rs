//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1514;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1515;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta405<F: Float>(t14767: F, t2477: F, t828: F, t1544: F, t2394: F, t10698: F, t10811: F, t4462: F, t4416: F, t808: F, t10886: F, t2703: F, t4458: F, t10678: F, t10682: F, t10687: F, t10692: F, t14759: F, t14761: F, t14765: F, t851: F, t10769: F, t836: F, t2749: F, t2746: F, t14494: F, t775: F, t14586: F, t10693: F, t10706: F, t10711: F, t10713: F, t10717: F, t10719: F, t10723: F, t10730: F, t10734: F, t10742: F, t2745: F, t4362: F) -> (F, F, F, F, F, F, F, F) {
        let (t14769, t14772, t14774, t14777, t14780, t14783) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1514::<F>(t14767, t2477, t828, t1544, t2394, t10698, t10811, t4462, t4416, t808, t10886, t2703, t4458);
        let t14784 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1515::<F>(t10678, t10682, t10687, t10692, t14759, t14761, t14765, t14769, t14774, t14777, t14780, t14783, t851);
        let (t14788, t14793, t14804, t14811) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1516::<F>(t10769, t828, t1544, t836, t2749, t2746, t14494, t775, t14586, t10693, t10706, t10711, t10713, t10717, t10719, t10723, t10730, t10734, t10742, t2745, t4362);
    (t14769, t14772, t14774, t14784, t14788, t14793, t14804, t14811)
}
