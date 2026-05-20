//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1967;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1968;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta540<F: Float>(t1294: F, t8201: F, t7652: F, t1287: F, t1794: F, t26931: F, t5284: F, t7660: F, t1215: F, t1295: F, t1770: F, t2144: F, t26889: F, t26895: F, t26918: F, t26976: F, t26979: F, t29213: F, t29217: F, t29220: F, t29224: F, t29227: F, t29233: F, t5216: F, t5231: F, t5423: F, t7602: F, t7629: F, t7636: F, t7643: F, t7659: F, t8202: F, t8213: F, t1214: F, t7637: F, t8197: F, t2142: F, t5497: F, t1209: F, t29135: F, t1774: F, t7627: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t29236, t29237, t29247, t29251, t29258) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1967::<F>(t1294, t8201, t7652, t1287, t1794, t26931, t5284, t7660, t1215, t1295, t1770, t2144, t26889, t26895, t26918, t26976, t26979, t29213, t29217, t29220, t29224, t29227, t29233, t5216, t5231, t5423, t7602, t7629, t7636, t7643, t7659, t8202, t8213);
        let (t29264, t29268, t29271) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1968::<F>(t1214, t7637, t8201, t8197, t2142, t5497);
        let (t29272, t29275, t29278) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1969::<F>(t29271, t7652, t1209, t29135, t1774, t7627);
    (t29236, t29237, t29247, t29251, t29258, t29264, t29268, t29271, t29272, t29275, t29278)
}
