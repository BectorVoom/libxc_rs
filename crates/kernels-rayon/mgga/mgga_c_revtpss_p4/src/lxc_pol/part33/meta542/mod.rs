//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1913;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta542(t29195: f64, t5465: f64, t1243: f64, t29192: f64, t2149: f64, t5480: f64, t3555: f64, t7635: f64, t460: f64, t8190: f64, t1204: f64, t1295: f64, t1775: f64, t1829: f64, t26889: f64, t26895: f64, t26922: f64, t26937: f64, t26999: f64, t27020: f64, t29160: f64, t29163: f64, t29167: f64, t29175: f64, t29179: f64, t29183: f64, t29187: f64, t29194: f64, t7636: f64, t7651: f64, t8192: f64, t8198: f64, t8209: f64, t1248: f64, t8201: f64, t1287: f64, t8197: f64, t1209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29196, t29199, t29200, t29201, t29204, t29207, t29210) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1913(t29195, t5465, t1243, t29192, t2149, t5480, t3555, t7635, t460, t8190, t1204, t1295, t1775, t1829, t26889, t26895, t26922, t26937, t26999, t27020, t29160, t29163, t29167, t29175, t29179, t29183, t29187, t29194, t7636, t7651, t8192, t8198, t8209);
        let (t29213, t29217, t29220) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1914(t1248, t8201, t1287, t8197, t1209, t8190);
    (t29196, t29199, t29200, t29201, t29204, t29207, t29210, t29213, t29217, t29220)
}
