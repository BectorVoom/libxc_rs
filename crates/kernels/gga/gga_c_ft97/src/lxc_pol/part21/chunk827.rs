//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 827/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk827<F: Float>(t53: F, t938: F, t72: F, t5591: F, t2247: F, t5578: F, t2258: F, t3052: F, t11233: F, t384: F, t73: F, t22632: F, t5598: F, t6445: F, t22652: F, t6427: F, t7839: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25792 = t938 * t53;
    let t25793 = t72 * t25792;
    let t25794 = t5591 * t25793;
    let t25797 = t5578 * t2247;
    let t25798 = t2258 * t3052;
    let t25799 = t25797 * t25798;
    let t25802 = t11233 * t384;
    let t25803 = t73 * t25802;
    let t25813 = t5598 * t22632 * t6445;
    let t25816 = t22652 * t938;
    let t25820 = t6427 * t7839;
    (t25792, t25793, t25794, t25798, t25799, t25802, t25803, t25813, t25816, t25820)
}
