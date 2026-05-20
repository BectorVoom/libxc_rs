//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta205 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta205<F: Float>(t2367: F, t625: F, t654: F, t2340: F, t665: F, t2339: F, t2366: F, t2269: F, t98: F, t99: F, t2350: F, t658: F, tau0: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10206, t10207, t10208, t10210, t10213, t10214, t10217, t10227, t10228) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk977::<F>(t2367, t625, t654, t2340, t665, t2339, t2366, t2269, t98, t99, t2350, t658, tau0);
    (t10206, t10207, t10208, t10210, t10213, t10214, t10217, t10227, t10228)
}
