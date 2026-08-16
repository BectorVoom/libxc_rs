//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta218 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1004;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta218<F: Float>(t10563: F, t162: F, t9544: F, t158: F, t755: F, t9586: F, t2619: F, t2622: F, t10552: F, t10554: F, t10557: F, t10560: F, t10562: F, t9333: F, t9394: F, t2390: F, t72: F, t757: F, t2629: F, t9863: F, t123: F, t752: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10564, t10565, t10566, t10568, t10570, t10571) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1004::<F>(t10563, t162, t9544, t158, t755, t9586, t2619, t2622, t10552, t10554, t10557, t10560, t10562, t9333, t9394);
        let (t10573, t10575, t10577, t10578) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1005::<F>(t2390, t72, t757, t2629, t9863, t123, t752);
    (t10564, t10565, t10566, t10568, t10570, t10571, t10573, t10575, t10577, t10578)
}
