//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1858;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1859;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1860;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta519<F: Float>(t14224: F, t25931: F, t72: F, t7920: F, t686: F, t25895: F, t25878: F, t25882: F, t25893: F, t25896: F, t25921: F, t25930: F, t27837: F, t27841: F, t27846: F, t27853: F, t27858: F, t27861: F, t27865: F, t27868: F, t7295: F, t7304: F, t7926: F, t1426: F, t27836: F, t7063: F, t7286: F, t7929: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t27869, t27872, t27873) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1858::<F>(t14224, t25931, t72, t7920, t686);
        let (t27874, t27876, t27879) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1859::<F>(t25895, t27873, t25878, t25882, t25893, t25896, t25921, t25930, t27837, t27841, t27846, t27853, t27858, t27861, t27865, t27868, t27869, t7295, t7304, t7926);
        let t27883 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1860::<F>(t1426, t27836);
        let (t27884, t27885, t27887, t27888) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1861::<F>(t27883, t7063, t7286, t72, t7929, t686);
    (t27869, t27872, t27873, t27874, t27876, t27879, t27883, t27884, t27885, t27887, t27888)
}
