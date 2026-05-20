//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2415;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2416;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta569<F: Float>(t1579: F, t4533: F, t2770: F, t212: F, t6041: F, t780: F, t689: F, t10498: F, t10501: F, t14474: F, t14479: F, t14484: F, t14486: F, t14985: F, t14989: F, t14992: F, t14995: F, t865: F, t6071: F, t886: F, t10673: F, t14675: F, t14690: F, t14703: F, t14705: F, t14712: F, t14715: F, t14716: F, t14722: F, t14726: F, t14730: F, t14734: F, t14494: F, t6035: F, t14791: F, t2703: F, t5985: F, t10905: F, t5989: F, t10678: F, t10687: F, t10692: F, t14736: F, t14744: F, t14759: F, t14761: F, t14765: F, t14777: F, t2745: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18312, t18313, t18316, t18317, t18322) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2415::<F>(t1579, t4533, t2770, t212, t6041, t780, t689, t10498, t10501, t14474, t14479, t14484, t14486, t14985, t14989, t14992, t14995, t865);
        let (t18324, t18330) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2416::<F>(t6071, t886, t2770, t10673, t14675, t14690, t14703, t14705, t14712, t14715, t14716, t14722, t14726, t14730, t14734);
        let (t18333, t18334, t18343) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2417::<F>(t14494, t6035, t14791, t2703, t5985, t10905, t5989, t10678, t10687, t10692, t14736, t14744, t14759, t14761, t14765, t14777, t2745);
    (t18312, t18313, t18316, t18317, t18322, t18324, t18330, t18333, t18334, t18343)
}
