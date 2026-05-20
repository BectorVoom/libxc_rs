//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1703;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1704;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta473<F: Float>(t2467: F, t26497: F, t25431: F, t26482: F, t2470: F, t7406: F, t7064: F, t136: F, t2066: F, t2457: F) -> (F, F, F, F, F, F) {
        let (t26498, t26500, t26506) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1703::<F>(t2467, t26497, t25431, t26482, t2470, t7406);
        let (t26508, t26518, t26519) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1704::<F>(t26506, t7064, t136, t2066, t2457);
    (t26498, t26500, t26506, t26508, t26518, t26519)
}
