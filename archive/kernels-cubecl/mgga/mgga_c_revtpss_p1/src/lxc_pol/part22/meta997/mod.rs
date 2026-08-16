//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta997 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3387;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta997<F: Float>(t15393: F, t15421: F, t15397: F, t52224: F, t2918: F, t2924: F, t6110: F, t11385: F, t2875: F, t6145: F, t198: F, t3336: F, t336: F, t63589: F, t63592: F, t63596: F, t63600: F, t63601: F, t63607: F, t63609: F, t63612: F, t63615: F, t63618: F, t15474: F, t1610: F, t2874: F, t11299: F, t11528: F, t19327: F, t19128: F, t934: F, t6142: F, t19330: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t63620, t63622, t63625, t63628, t63629) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3387::<F>(t15393, t15421, t15397, t52224, t2918, t2924, t6110, t11385, t2875, t6145, t198, t3336, t336, t63589, t63592, t63596, t63600, t63601, t63607, t63609, t63612, t63615, t63618);
        let (t63633, t63636, t63638, t63641, t63644, t63647) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3388::<F>(t15474, t1610, t2874, t11299, t2918, t6145, t11528, t19327, t19128, t934, t6142, t19330, t2875);
    (t63620, t63622, t63625, t63628, t63629, t63633, t63636, t63638, t63641, t63644, t63647)
}
