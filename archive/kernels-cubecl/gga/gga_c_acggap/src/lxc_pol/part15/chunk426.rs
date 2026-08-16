//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 426/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk426<F: Float>(t301: F, t599: F, t142: F, t2030: F, t130: F, t228: F) -> (F, F, F, F) {
    let t2031 = t599 * t301;
    let t2032 = t142 * t2031;
    let t2033 = t2030 * t2032;
    let t2035 = t130 * t228;
    (t2031, t2032, t2033, t2035)
}
