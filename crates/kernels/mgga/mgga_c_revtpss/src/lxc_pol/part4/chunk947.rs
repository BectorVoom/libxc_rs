//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 947/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk947<F: Float>(t231: F, t2783: F, t836: F, t860: F, t2782: F, t251: F, t2645: F, t10111: F, t22: F, t870: F, t2723: F, t10115: F, t253: F, t233: F, t2760: F, t869: F) -> (F, F, F, F, F, F) {
    let t10929 = t2783 * t860 * t836 * t231;
    let t10930 = t2782 * t10929;
    let t10932 = t251 * t2645;
    let t10934 = t2783 * t10932 * t231;
    let t10935 = t2782 * t10934;
    let t10939 = 0.19637199382202157274e-3 * t10111 * t870 * t22;
    let t10943 = t2723 * t2645;
    let t10948 = 0.11044544084478153697e-3 * t10115 * t253;
    let t10959 = t233 * t2760;
    let t10960 = t869 * t10959;
    (t10930, t10935, t10939, t10943, t10948, t10960)
}
