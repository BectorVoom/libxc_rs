//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 798/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk798<F: Float>(t4056: F, t550: F, t543: F, t3992: F, t2661: F, t240: F, t4000: F, t4003: F, t9768: F, t532: F, t549: F, t72: F, t828: F, t9400: F, t595: F, t66: F) -> (F, F, F, F, F, F) {
    let t9929 = t550 * t4056;
    let t9930 = t9929 * t543;
    let t9931 = t3992 * t9930;
    let t9932 = t2661 * t9931;
    let t9934 = t4000 * t240;
    let t9935 = t9768 * t4003;
    let t9936 = t9934 * t9935;
    let t9937 = t2661 * t9936;
    let t9940 = 1.0 / t549 / t532;
    let t9941 = t240 * t9940;
    let t9942 = t9941 * t72;
    let t9944 = t9942 * t828 * t9400;
    let t9948 = 1.0 / t66 / t595;
    (t9930, t9932, t9935, t9937, t9944, t9948)
}
