//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1785;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta409(t1579: f64, t4533: f64, t2770: f64, t212: f64, t6041: f64, t780: f64, t689: f64, t10498: f64, t10501: f64, t14474: f64, t14479: f64, t14484: f64, t14486: f64, t14985: f64, t14989: f64, t14992: f64, t14995: f64, t865: f64, t6071: f64, t886: f64, t10673: f64, t14675: f64, t14690: f64, t14703: f64, t14705: f64, t14712: f64, t14715: f64, t14716: f64, t14722: f64, t14726: f64, t14730: f64, t14734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18312, t18313, t18316, t18317, t18318, t18322) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1785(t1579, t4533, t2770, t212, t6041, t780, t689, t10498, t10501, t14474, t14479, t14484, t14486, t14985, t14989, t14992, t14995, t865);
        let (t18323, t18324, t18330) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1786(t6071, t886, t2770, t10673, t14675, t14690, t14703, t14705, t14712, t14715, t14716, t14722, t14726, t14730, t14734);
    (t18312, t18313, t18316, t18317, t18318, t18322, t18323, t18324, t18330)
}
