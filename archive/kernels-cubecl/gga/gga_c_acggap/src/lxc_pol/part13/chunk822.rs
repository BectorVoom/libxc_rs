//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 822/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk822<F: Float>(t4256: F, t8915: F, t7450: F, t2297: F, t372: F, t4262: F, t2030: F, t2288: F, t301: F, t1016: F, t142: F, t2060: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8916 = t4256 * t8915;
    let t8917 = t7450 * t8916;
    let t8919 = t2297 * t372;
    let t8920 = t4262 * t8919;
    let t8921 = t2030 * t8920;
    let t8923 = t2288 * t301;
    let t8924 = t4262 * t8923;
    let t8925 = t2030 * t8924;
    let t8927 = t142 * t1016;
    let t8928 = t2288 * t372;
    let t8929 = t8927 * t8928;
    let t8930 = t2060 * t8929;
    (t8916, t8917, t8919, t8920, t8921, t8923, t8924, t8925, t8927, t8928, t8929, t8930)
}
