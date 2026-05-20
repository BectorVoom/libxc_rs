//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta518<F: Float>(t25410: F, t93320: F, t7063: F, t860: F, t25374: F, t11007: F, t1955: F, t7056: F, t93189: F, t93169: F, t1113: F, t2411: F) -> (F, F, F, F, F, F, F, F) {
        let (t93321, t93342, t93349, t93364, t93371, t93374, t93377, t94245) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1840::<F>(t25410, t93320, t7063, t860, t25374, t11007, t1955, t7056, t93189, t93169, t1113, t2411);
    (t93321, t93342, t93349, t93364, t93371, t93374, t93377, t94245)
}
