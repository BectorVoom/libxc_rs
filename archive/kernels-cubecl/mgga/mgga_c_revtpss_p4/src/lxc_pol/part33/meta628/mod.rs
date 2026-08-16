//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2072;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta628<F: Float>(t25375: F, t99125: F, t25387: F, t27182: F, t686: F, t72: F, t2435: F, t27334: F, t10867: F, t1949: F, t14485: F, t25399: F, t27195: F, t1955: F, t27198: F, t2769: F, t2470: F, t27278: F, t7064: F, t10073: F, t25402: F, t7056: F, t7759: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t99127, t99147, t99161, t99163, t99166, t99174, t99186) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2072::<F>(t25375, t99125, t25387, t27182, t686, t72, t2435, t27334, t10867, t1949, t14485, t25399);
        let (t99188, t99191, t99201, t99202, t99206) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2073::<F>(t2435, t27195, t1955, t27198, t2769, t2470, t27278, t7064, t10073, t25402, t7056, t7759);
    (t99127, t99147, t99161, t99163, t99166, t99174, t99186, t99188, t99191, t99201, t99202, t99206)
}
