//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2158/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2158<F: Float>(t107922: F, t107963: F, t108001: F, t108047: F, t22279: F, t28167: F, t8996: F, t29506: F, t7313: F, t1843: F, t28042: F, t651: F) -> (F, F, F, F) {
    let t108049 = t107922 + t107963 + t108001 + t108047;
    let t108067 = F::new(12.0) * t28167 * t8996 * t22279;
    let t108068 = t29506 * t7313;
    let t108076 = F::new(4.0) * t651 * t1843 * t28042;
    (t108049, t108067, t108068, t108076)
}
