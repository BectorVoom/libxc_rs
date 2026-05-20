//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1922;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta591<F: Float>(t103181: F, t28313: F, t93317: F, t4534: F, t689: F, t7384: F, t213: F, t28340: F, t26544: F, t27213: F, t14983: F, t26497: F, t14485: F, t4481: F, t95743: F, t10073: F, t25402: F, t7056: F, t7997: F, t26519: F, t98867: F, t98937: F, t98949: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t103182, t103184, t103196, t103212, t103216, t103219) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1922::<F>(t103181, t28313, t93317, t4534, t689, t7384, t213, t28340, t26544, t27213, t14983, t26497);
        let (t103220, t103224, t103234, t103240, t103247, t103254) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1923::<F>(t14485, t26497, t4481, t95743, t10073, t25402, t7056, t7997, t26519, t98867, t98937, t98949);
    (t103182, t103184, t103196, t103212, t103216, t103219, t103220, t103224, t103234, t103240, t103247, t103254)
}
