//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1099;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1100;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1101;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta248(t1280: f64, t6573: f64, t1287: f64, t6688: f64, t1774: f64, t5486: f64, t6587: f64, t487: f64, t6628: f64, t3769: f64, t1794: f64, t1811: f64, t6622: f64, t3783: f64, t489: f64, t6695: f64, t1234: f64, t1285: f64, t1770: f64, t1818: f64, t1822: f64, t1825: f64, t3670: f64, t3755: f64, t3767: f64, t3782: f64, t460: f64, t490: f64, t5326: f64, t5436: f64, t6564: f64, t1277: f64, t1210: f64, t1274: f64, t1775: f64, t1813: f64, t1829: f64, t3567: f64, t495: f64, t5220: f64, t5225: f64, t5251: f64, t5417: f64, t6574: f64, t6580: f64, t6588: f64, t6697: f64, t6703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6714, t6717, t6720, t6723, t6726, t6727, t6731) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1099(t1280, t6573, t1287, t6688, t1774, t5486, t6587, t487, t6628, t3769, t1794, t1811);
        let (t6735, t6738, t6741, t6744) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1100(t1287, t487, t6622, t3783, t6726, t489, t6695, t1234, t1285, t1770, t1818, t1822, t1825, t3670, t3755, t3767, t3782, t460, t490, t5326, t5436, t6564, t6714, t6717, t6720, t6723, t6727, t6731);
        let t6745 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1101(t1277, t6744);
        let t6748 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1102(t1210, t1274, t1770, t1775, t1813, t1829, t3567, t460, t495, t5220, t5225, t5251, t5417, t6564, t6574, t6580, t6588, t6697, t6703, t6745);
    (t6714, t6717, t6720, t6723, t6727, t6731, t6735, t6738, t6741, t6744, t6745, t6748)
}
