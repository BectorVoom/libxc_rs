//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1979/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1979<F: Float>(t5627: F, t8996: F, t28167: F, t1310: F, t1453: F, t28050: F, t28053: F, t28058: F, t28060: F, t28062: F, t28065: F, t28069: F, t28160: F, t28165: F, t4248: F, t508: F, t649: F, t651: F, t7007: F, t7725: F, t7883: F, t7894: F) -> (F, F) {
    let t28168 = t8996 * t5627;
    let t28170 = F::new(6.0) * t28167 * t28168;
    let t28171 = -t1310 * t7725 + t1453 * t7894 - F::new(2.0) * t28050 * t651 - F::new(2.0) * t28053 * t651 - t28160 * t508 - F::new(2.0) * t4248 * t7007 - t649 * t7883 - t28058 - t28060 - t28062 - t28065 - t28069 + t28165 + t28170;
    (t28168, t28171)
}
