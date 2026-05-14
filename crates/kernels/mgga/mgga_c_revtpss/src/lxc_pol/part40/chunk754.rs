//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 754/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk754<F: Float>(t1369: F, t794: F, t1372: F, t124: F, t3889: F, t800: F, t2453: F, t546: F) -> (F, F, F, F) {
    let t3957 = t794 * t1369;
    let t3958 = t3957 * t1372;
    let t3960 = t124 * t3889;
    let t3961 = t800 * t3960;
    let t3964 = t2453 * t546;
    (t3957, t3958, t3961, t3964)
}
