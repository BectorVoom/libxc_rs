//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 811/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk811<F: Float>(t2288: F, t301: F, t4262: F, t2030: F, t1016: F, t142: F, t372: F, t2060: F, t336: F, t4630: F, t570: F, t2020: F, t2260: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8923 = t2288 * t301;
    let t8924 = t4262 * t8923;
    let t8925 = t2030 * t8924;
    let t8927 = t142 * t1016;
    let t8928 = t2288 * t372;
    let t8929 = t8927 * t8928;
    let t8930 = t2060 * t8929;
    let t8942 = t336 * t4630;
    let t8943 = t570 * t8942;
    let t8945 = t2020 * t2260;
    (t8923, t8924, t8925, t8927, t8928, t8929, t8930, t8942, t8943, t8945)
}
