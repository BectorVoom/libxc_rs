//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 951/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk951<F: Float>(t53: F, t938: F, t72: F, t5591: F, t2247: F, t5578: F, t2258: F, t3052: F, t11233: F, t384: F) -> (F, F, F, F, F, F) {
    let t25792 = t938 * t53;
    let t25793 = t72 * t25792;
    let t25794 = t5591 * t25793;
    let t25797 = t5578 * t2247;
    let t25798 = t2258 * t3052;
    let t25799 = t25797 * t25798;
    let t25802 = t11233 * t384;
    (t25793, t25794, t25797, t25798, t25799, t25802)
}
