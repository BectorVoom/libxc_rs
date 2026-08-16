//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta299<F: Float>(t240: F, t4000: F, t532: F, t549: F, t72: F, t595: F, t66: F, t247: F, t550: F, t548: F, t4010: F, t245: F) -> (F, F, F, F, F, F, F) {
        let (t9934, t9942, t9949, t9951, t9953, t9954, t9955) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1290::<F>(t240, t4000, t532, t549, t72, t595, t66, t247, t550, t548, t4010, t245);
    (t9934, t9942, t9949, t9951, t9953, t9954, t9955)
}
