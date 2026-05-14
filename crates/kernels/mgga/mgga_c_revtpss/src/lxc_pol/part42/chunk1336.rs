//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1336/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1336<F: Float>(t1453: F, t8406: F, t1843: F, t8320: F, t1310: F, t31027: F, t8395: F, t28036: F, t8311: F, t1513: F, t661: F, t8315: F, t4287: F, t625: F, t8399: F, t109: F, t55: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31401 = t8406 * t1453;
    let t31403 = t1843 * t8320;
    let t31407 = t1310 * t8406;
    let t31415 = t31027 * t8395;
    let t31417 = t8311 * t28036;
    let t31420 = t1513 * t661;
    let t31421 = t8315 * t31420;
    let t31424 = t8311 * t4287;
    let t31427 = t625 * t8399;
    let t31429 = t55 * t109;
    (t31401, t31403, t31407, t31415, t31417, t31420, t31421, t31424, t31427, t31429)
}
