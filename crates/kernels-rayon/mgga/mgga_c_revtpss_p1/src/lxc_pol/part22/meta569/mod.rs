//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2415;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2416;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta569(t1579: f64, t4533: f64, t2770: f64, t212: f64, t6041: f64, t780: f64, t689: f64, t10498: f64, t10501: f64, t14474: f64, t14479: f64, t14484: f64, t14486: f64, t14985: f64, t14989: f64, t14992: f64, t14995: f64, t865: f64, t6071: f64, t886: f64, t10673: f64, t14675: f64, t14690: f64, t14703: f64, t14705: f64, t14712: f64, t14715: f64, t14716: f64, t14722: f64, t14726: f64, t14730: f64, t14734: f64, t14494: f64, t6035: f64, t14791: f64, t2703: f64, t5985: f64, t10905: f64, t5989: f64, t10678: f64, t10687: f64, t10692: f64, t14736: f64, t14744: f64, t14759: f64, t14761: f64, t14765: f64, t14777: f64, t2745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18312, t18313, t18316, t18317, t18322) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2415(t1579, t4533, t2770, t212, t6041, t780, t689, t10498, t10501, t14474, t14479, t14484, t14486, t14985, t14989, t14992, t14995, t865);
        let (t18324, t18330) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2416(t6071, t886, t2770, t10673, t14675, t14690, t14703, t14705, t14712, t14715, t14716, t14722, t14726, t14730, t14734);
        let (t18333, t18334, t18343) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2417(t14494, t6035, t14791, t2703, t5985, t10905, t5989, t10678, t10687, t10692, t14736, t14744, t14759, t14761, t14765, t14777, t2745);
    (t18312, t18313, t18316, t18317, t18322, t18324, t18330, t18333, t18334, t18343)
}
