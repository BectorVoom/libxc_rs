//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 971/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk971<F: Float>(t37935: F, t565: F, t10711: F, t10734: F, t547: F, t546: F, t10737: F, t255: F, t6319: F) -> (F, F, F, F, F) {
    let t37939 = t565 * t37935;
    let t37940 = t37939 * t10711;
    let t37942 = t547 * t10734;
    let t37943 = t546 * t37942;
    let t37945 = t10737 * t255 * t6319;
    (t37939, t37940, t37942, t37943, t37945)
}
