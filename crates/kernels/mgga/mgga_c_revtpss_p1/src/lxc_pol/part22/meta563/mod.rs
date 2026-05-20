//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2399;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta563<F: Float>(t487: F, t5216: F, t1211: F, t16771: F, t16775: F, t1210: F, t1215: F, t12603: F, t1295: F, t18043: F, t18047: F, t18054: F, t18059: F, t18062: F, t1813: F, t1829: F, t3552: F, t3556: F, t3567: F, t3569: F, t3572: F, t3585: F, t5220: F, t5246: F, t5251: F, t5423: F, t1277: F, t1774: F, t3790: F, t1204: F, t1811: F, t16750: F, t1209: F, t5412: F) -> (F, F, F, F, F, F, F, F) {
        let (t18065, t18070, t18073, t18080) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2399::<F>(t487, t5216, t1211, t16771, t16775, t1210, t1215, t12603, t1295, t18043, t18047, t18054, t18059, t18062, t1813, t1829, t3552, t3556, t3567, t3569, t3572, t3585, t5220, t5246, t5251, t5423);
        let (t18084, t18087, t18090, t18097) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2400::<F>(t1277, t1774, t3790, t1204, t1811, t1211, t16750, t1209, t5412);
    (t18065, t18070, t18073, t18080, t18084, t18087, t18090, t18097)
}
