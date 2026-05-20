//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta211<F: Float>(t300: F, t4715: F, t4683: F, t1626: F, t983: F, t1642: F, t3022: F, t1633: F, t2986: F, t974: F, t981: F, t4707: F, t964: F, t973: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4716, t4718, t4719, t4721, t4723, t4724, t4725, t4727, t4729) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk855::<F>(t300, t4715, t4683, t1626, t983, t1642, t3022, t1633, t2986, t974, t981, t4707, t964, t973);
    (t4716, t4718, t4719, t4721, t4723, t4724, t4725, t4727, t4729)
}
