//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2073;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta598<F: Float>(t10073: F, t1444: F, t2029: F, t25929: F, t26041: F, t9664: F, t2030: F, t47567: F, t26069: F, t94806: F, t1426: F, t94609: F, t7063: F, t7286: F, t7289: F, t94810: F, t26054: F, t9686: F, t25877: F, t94801: F, t25881: F, t1419: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t94857, t94865, t94867, t94876, t94878) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2073::<F>(t10073, t1444, t2029, t25929, t26041, t9664, t2030, t47567, t26069, t94806, t1426, t94609);
        let (t94880, t94882, t94884, t94886, t94887, t94889) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2074::<F>(t7063, t94878, t7286, t7289, t94810, t26054, t9686, t25877, t94801, t25881, t1419, t786);
    (t94857, t94865, t94867, t94876, t94878, t94880, t94882, t94884, t94886, t94887, t94889)
}
