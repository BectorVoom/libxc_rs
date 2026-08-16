//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 787/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk787<F: Float>(t2030: F, t8920: F, t2288: F, t301: F, t4262: F, t1016: F, t142: F, t372: F, t2060: F, t336: F, t4630: F, t570: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8921 = t2030 * t8920;
    let t8923 = t2288 * t301;
    let t8924 = t4262 * t8923;
    let t8925 = t2030 * t8924;
    let t8927 = t142 * t1016;
    let t8928 = t2288 * t372;
    let t8929 = t8927 * t8928;
    let t8930 = t2060 * t8929;
    let t8942 = t336 * t4630;
    let t8943 = t570 * t8942;
    (t8921, t8923, t8924, t8925, t8927, t8928, t8929, t8930, t8942, t8943)
}
