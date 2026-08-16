//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2239;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2240;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta661<F: Float>(t1353: F, t30122: F, t28167: F, t8717: F, t2014: F, t25190: F, t29494: F, t27833: F, t7901: F, t28020: F, t5542: F, t1450: F, t21969: F, t7237: F, t35669: F, t5627: F, t29996: F, t7235: F, t22483: F, t7312: F, t109078: F, t109081: F, t109087: F, t109090: F, t109092: F, t109095: F, t109099: F, t109103: F, t1843: F, t1911: F, t28160: F, t28230: F, t5517: F, t7725: F, t28172: F, t28176: F, t29498: F, t94345: F, t29583: F, t2322: F, t30128: F, t4254: F, t1936: F, t21658: F, t651: F) -> (F, F, F, F, F, F, F) {
        let (t109107, t109110, t109112, t109117, t109118) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2239::<F>(t1353, t30122, t28167, t8717, t2014, t25190, t29494, t27833, t7901, t28020, t5542, t1450, t21969);
        let t109129 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2240::<F>(t109118, t2014, t7237, t28167, t35669, t5627, t29996, t7235, t22483, t7312, t109078, t109081, t109087, t109090, t109092, t109095, t109099, t109103, t109107, t109110, t109112, t109117, t1843, t1911, t28160, t28230, t5517, t7725);
        let (t109135, t109138, t109140, t109142, t109144, t109147) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2241::<F>(t2014, t28172, t28176, t29498, t94345, t29583, t7235, t2322, t30128, t4254, t1936, t21658, t651);
    (t109129, t109135, t109138, t109140, t109142, t109144, t109147)
}
