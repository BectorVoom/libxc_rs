//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1964;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta538<F: Float>(t1769: F, t7627: F, t7637: F, t11239: F, t1276: F, t3596: F, t2149: F, t29157: F, t3153: F, t5465: F, t1243: F, t5480: F, t3555: F, t7635: F, t460: F, t8190: F, t1204: F, t1295: F, t1775: F, t1829: F, t26889: F, t26895: F, t26922: F, t26937: F, t26999: F, t27020: F, t29160: F, t29163: F, t29167: F, t29175: F, t29179: F, t29183: F, t7636: F, t7651: F, t8192: F, t8198: F, t8209: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29186, t29187, t29192, t29193, t29194, t29195) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1964::<F>(t1769, t7627, t7637, t11239, t1276, t3596, t2149, t29157, t3153);
        let (t29196, t29199, t29200, t29201, t29204, t29207, t29210) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1965::<F>(t29195, t5465, t1243, t29192, t2149, t5480, t3555, t7635, t460, t8190, t1204, t1295, t1775, t1829, t26889, t26895, t26922, t26937, t26999, t27020, t29160, t29163, t29167, t29175, t29179, t29183, t29187, t29194, t7636, t7651, t8192, t8198, t8209);
    (t29186, t29187, t29193, t29194, t29195, t29196, t29199, t29200, t29201, t29204, t29207, t29210)
}
