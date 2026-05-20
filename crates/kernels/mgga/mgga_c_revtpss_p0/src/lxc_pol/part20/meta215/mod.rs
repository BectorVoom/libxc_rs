//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta215<F: Float>(t10518: F, t2798: F, t2722: F, t675: F, t231: F, t268: F, t251: F, t4503: F, t786: F, t2723: F, t2453: F, t2797: F) -> (F, F, F, F, F, F, F, F) {
        let (t10519, t10523, t10524, t10529, t10530, t10532, t10533, t10535) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1000::<F>(t10518, t2798, t2722, t675, t231, t268, t251, t4503, t786, t2723, t2453, t2797);
    (t10519, t10523, t10524, t10529, t10530, t10532, t10533, t10535)
}
