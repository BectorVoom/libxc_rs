//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 537/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk537<F: Float>(t1265: F, t857: F, t1210: F, t315: F, t323: F, t188: F, t3645: F, t119: F, t441: F, t862: F, t865: F, t1212: F, t310: F, t447: F, t150: F, t443: F, t848: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3856 = t857 * t1265;
    let t3858 = t315 * t1210;
    let t3859 = t3858 * t323;
    let t3862 = 0.65854491829355115987e0 * t3645 * t188;
    let t3865 = t119 * t1210;
    let t3868 = t862 * t441;
    let t3869 = t3868 * t865;
    let t3871 = t310 * t1212;
    let t3873 = t447 * t447;
    let t3874 = 1.0 / t3873;
    let t3875 = t150 * t3874;
    let t3880 = t848 * t443;
    (t3856, t3859, t3862, t3865, t3869, t3871, t3873, t3874, t3875, t3880)
}
