//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1915;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1916;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta543(t1294: f64, t7652: f64, t8197: f64, t1770: f64, t2142: f64, t1214: f64, t7637: f64, t8190: f64, t8201: f64, t1287: f64, t1794: f64, t26931: f64, t5284: f64, t7660: f64, t1215: f64, t1295: f64, t2144: f64, t26889: f64, t26895: f64, t26918: f64, t26976: f64, t26979: f64, t29213: f64, t29217: f64, t29220: f64, t5216: f64, t5231: f64, t5423: f64, t7602: f64, t7629: f64, t7636: f64, t7643: f64, t7659: f64, t8202: f64, t8213: f64, t5497: f64, t1209: f64, t29135: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29224, t29227, t29233, t29237, t29247) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1915(t1294, t7652, t8197, t1770, t2142, t1214, t7637, t8190, t8201, t1287, t1794, t26931);
        let (t29251, t29258) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1916(t1287, t5284, t7660, t1215, t1295, t1770, t2144, t26889, t26895, t26918, t26976, t26979, t29213, t29217, t29220, t29224, t29227, t29233, t29237, t29247, t5216, t5231, t5423, t7602, t7629, t7636, t7643, t7659, t8202, t8213);
        let (t29264, t29268, t29271, t29272, t29275) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1917(t1214, t7637, t8201, t8197, t2142, t5497, t7652, t1209, t29135);
    (t29224, t29227, t29233, t29237, t29247, t29251, t29258, t29264, t29268, t29271, t29272, t29275)
}
