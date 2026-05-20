//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta540<F: Float>(t25387: F, t95628: F, t11015: F, t7388: F, t92975: F, t92988: F, t92995: F, t92997: F, t92999: F, t93007: F, t93012: F, t93020: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t95629, t95632, t95666, t95671, t95673, t95674, t95675, t95678, t95680, t95684) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1851::<F>(t25387, t95628, t11015, t7388, t92975, t92988, t92995, t92997, t92999, t93007, t93012, t93020);
    (t95629, t95632, t95666, t95671, t95673, t95674, t95675, t95678, t95680, t95684)
}
