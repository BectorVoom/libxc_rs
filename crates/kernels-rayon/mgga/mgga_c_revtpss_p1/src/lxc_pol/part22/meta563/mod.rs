//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2399;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta563(t487: f64, t5216: f64, t1211: f64, t16771: f64, t16775: f64, t1210: f64, t1215: f64, t12603: f64, t1295: f64, t18043: f64, t18047: f64, t18054: f64, t18059: f64, t18062: f64, t1813: f64, t1829: f64, t3552: f64, t3556: f64, t3567: f64, t3569: f64, t3572: f64, t3585: f64, t5220: f64, t5246: f64, t5251: f64, t5423: f64, t1277: f64, t1774: f64, t3790: f64, t1204: f64, t1811: f64, t16750: f64, t1209: f64, t5412: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18065, t18070, t18073, t18080) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2399(t487, t5216, t1211, t16771, t16775, t1210, t1215, t12603, t1295, t18043, t18047, t18054, t18059, t18062, t1813, t1829, t3552, t3556, t3567, t3569, t3572, t3585, t5220, t5246, t5251, t5423);
        let (t18084, t18087, t18090, t18097) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2400(t1277, t1774, t3790, t1204, t1811, t1211, t16750, t1209, t5412);
    (t18065, t18070, t18073, t18080, t18084, t18087, t18090, t18097)
}
