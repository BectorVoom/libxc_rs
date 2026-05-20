//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta183 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta183<F: Float>(t240: F, t9720: F, t550: F, t268: F, t9718: F, t64: F, t8779: F, t159: F, t535: F, t2236: F, t65: F, t235: F) -> (F, F, F, F, F, F, F, F) {
        let (t9721, t9722, t9725, t9726, t9727, t9729, t9731, t9732) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk927::<F>(t240, t9720, t550, t268, t9718, t64, t8779, t159, t535, t2236, t65, t235);
    (t9721, t9722, t9725, t9726, t9727, t9729, t9731, t9732)
}
