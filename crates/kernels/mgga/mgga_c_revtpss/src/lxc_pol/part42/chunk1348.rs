//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1348/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1348<F: Float>(t116929: F, t8402: F, t116926: F, t8395: F, t2289: F, t8399: F, t31027: F, t31424: F, t31440: F, t31032: F, t31444: F, t108: F, t1513: F, t116912: F, t31417: F, t31421: F) -> (F, F, F, F, F, F, F, F, F) {
    let t117936 = t116929 * t8402;
    let t117938 = t116926 * t8395;
    let t117940 = t2289 * t8399;
    let t117943 = 4.0 / 3.0 * t31027 * t31424;
    let t117976 = 20.0 / 9.0 * t31027 * t31440;
    let t117978 = 20.0 / 27.0 * t31032 * t31444;
    let t117997 = t108 * t1513;
    let t118009 = 4.0 * t116912 * t31417;
    let t118011 = 20.0 / 9.0 * t31027 * t31421;
    (t117936, t117938, t117940, t117943, t117976, t117978, t117997, t118009, t118011)
}
