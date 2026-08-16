//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1964;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta538(t1769: f64, t7627: f64, t7637: f64, t11239: f64, t1276: f64, t3596: f64, t2149: f64, t29157: f64, t3153: f64, t5465: f64, t1243: f64, t5480: f64, t3555: f64, t7635: f64, t460: f64, t8190: f64, t1204: f64, t1295: f64, t1775: f64, t1829: f64, t26889: f64, t26895: f64, t26922: f64, t26937: f64, t26999: f64, t27020: f64, t29160: f64, t29163: f64, t29167: f64, t29175: f64, t29179: f64, t29183: f64, t7636: f64, t7651: f64, t8192: f64, t8198: f64, t8209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29186, t29187, t29192, t29193, t29194, t29195) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1964(t1769, t7627, t7637, t11239, t1276, t3596, t2149, t29157, t3153);
        let (t29196, t29199, t29200, t29201, t29204, t29207, t29210) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1965(t29195, t5465, t1243, t29192, t2149, t5480, t3555, t7635, t460, t8190, t1204, t1295, t1775, t1829, t26889, t26895, t26922, t26937, t26999, t27020, t29160, t29163, t29167, t29175, t29179, t29183, t29187, t29194, t7636, t7651, t8192, t8198, t8209);
    (t29186, t29187, t29193, t29194, t29195, t29196, t29199, t29200, t29201, t29204, t29207, t29210)
}
