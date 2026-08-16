//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta563<F: Float>(t13869: F, t7271: F, t13878: F, t25972: F, t13967: F, t26028: F, t13937: F, t13981: F, t2689: F, t27936: F, t13857: F, t94564: F) -> (F, F, F, F, F, F, F) {
        let (t98204, t98206, t98211, t98213, t98215, t98218, t98220) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1907::<F>(t13869, t7271, t13878, t25972, t13967, t26028, t13937, t13981, t2689, t27936, t13857, t94564);
    (t98204, t98206, t98211, t98213, t98215, t98218, t98220)
}
