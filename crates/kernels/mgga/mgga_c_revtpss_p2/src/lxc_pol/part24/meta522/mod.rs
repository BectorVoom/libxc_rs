//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1551;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta522<F: Float>(t1247: F, t24772: F, t3172: F, t20819: F, t5292: F, t17505: F, t20783: F, t1260: F, t24699: F, t21242: F, t5378: F, t1785: F, t21271: F, t1261: F, t24248: F, t247: F, t3634: F, t21233: F, t5381: F, t17401: F, t20926: F, t24770: F, t73: F, t12916: F, t24752: F, t3718: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82553, t82555, t82560, t82565, t82595, t82597) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1551::<F>(t1247, t24772, t3172, t20819, t5292, t17505, t20783, t1260, t24699, t21242, t5378, t1785, t21271);
        let (t82603, t82656, t82678, t82725, t82749) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1552::<F>(t1261, t24248, t247, t3634, t21233, t5381, t17401, t20926, t24770, t73, t12916, t24752, t3718);
    (t82553, t82555, t82560, t82565, t82595, t82597, t82603, t82656, t82678, t82725, t82749)
}
