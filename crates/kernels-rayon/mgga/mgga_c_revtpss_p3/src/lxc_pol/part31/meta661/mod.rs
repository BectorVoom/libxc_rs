//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2239;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2240;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta661(t1353: f64, t30122: f64, t28167: f64, t8717: f64, t2014: f64, t25190: f64, t29494: f64, t27833: f64, t7901: f64, t28020: f64, t5542: f64, t1450: f64, t21969: f64, t7237: f64, t35669: f64, t5627: f64, t29996: f64, t7235: f64, t22483: f64, t7312: f64, t109078: f64, t109081: f64, t109087: f64, t109090: f64, t109092: f64, t109095: f64, t109099: f64, t109103: f64, t1843: f64, t1911: f64, t28160: f64, t28230: f64, t5517: f64, t7725: f64, t28172: f64, t28176: f64, t29498: f64, t94345: f64, t29583: f64, t2322: f64, t30128: f64, t4254: f64, t1936: f64, t21658: f64, t651: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t109107, t109110, t109112, t109117, t109118) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2239(t1353, t30122, t28167, t8717, t2014, t25190, t29494, t27833, t7901, t28020, t5542, t1450, t21969);
        let t109129 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2240(t109118, t2014, t7237, t28167, t35669, t5627, t29996, t7235, t22483, t7312, t109078, t109081, t109087, t109090, t109092, t109095, t109099, t109103, t109107, t109110, t109112, t109117, t1843, t1911, t28160, t28230, t5517, t7725);
        let (t109135, t109138, t109140, t109142, t109144, t109147) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2241(t2014, t28172, t28176, t29498, t94345, t29583, t7235, t2322, t30128, t4254, t1936, t21658, t651);
    (t109129, t109135, t109138, t109140, t109142, t109144, t109147)
}
