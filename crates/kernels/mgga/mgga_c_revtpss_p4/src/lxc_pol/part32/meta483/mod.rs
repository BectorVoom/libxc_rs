//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta483<F: Float>(t2661: F, t27928: F, t13846: F, t1941: F, t13877: F, t2018: F, t5617: F, t807: F, t241: F, t25981: F, t820: F) -> (F, F, F, F, F, F) {
        let (t27929, t27932, t27933, t27936, t27937, t27940) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1728::<F>(t2661, t27928, t13846, t1941, t13877, t2018, t5617, t807, t241, t25981, t820);
    (t27929, t27932, t27933, t27936, t27937, t27940)
}
