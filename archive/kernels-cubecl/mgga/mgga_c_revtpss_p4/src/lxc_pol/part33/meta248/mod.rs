//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1099;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1100;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1101;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta248<F: Float>(t1280: F, t6573: F, t1287: F, t6688: F, t1774: F, t5486: F, t6587: F, t487: F, t6628: F, t3769: F, t1794: F, t1811: F, t6622: F, t3783: F, t489: F, t6695: F, t1234: F, t1285: F, t1770: F, t1818: F, t1822: F, t1825: F, t3670: F, t3755: F, t3767: F, t3782: F, t460: F, t490: F, t5326: F, t5436: F, t6564: F, t1277: F, t1210: F, t1274: F, t1775: F, t1813: F, t1829: F, t3567: F, t495: F, t5220: F, t5225: F, t5251: F, t5417: F, t6574: F, t6580: F, t6588: F, t6697: F, t6703: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6714, t6717, t6720, t6723, t6726, t6727, t6731) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1099::<F>(t1280, t6573, t1287, t6688, t1774, t5486, t6587, t487, t6628, t3769, t1794, t1811);
        let (t6735, t6738, t6741, t6744) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1100::<F>(t1287, t487, t6622, t3783, t6726, t489, t6695, t1234, t1285, t1770, t1818, t1822, t1825, t3670, t3755, t3767, t3782, t460, t490, t5326, t5436, t6564, t6714, t6717, t6720, t6723, t6727, t6731);
        let t6745 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1101::<F>(t1277, t6744);
        let t6748 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1102::<F>(t1210, t1274, t1770, t1775, t1813, t1829, t3567, t460, t495, t5220, t5225, t5251, t5417, t6564, t6574, t6580, t6588, t6697, t6703, t6745);
    (t6714, t6717, t6720, t6723, t6727, t6731, t6735, t6738, t6741, t6744, t6745, t6748)
}
