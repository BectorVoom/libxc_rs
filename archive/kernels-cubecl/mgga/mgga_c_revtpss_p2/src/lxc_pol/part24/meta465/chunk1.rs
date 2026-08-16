//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1439/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1439<F: Float>(t2609: F, t2611: F, t5819: F, t14440: F, t4311: F, t123: F, t2630: F, t5941: F, t18555: F, t2619: F, t18562: F, t2516: F) -> (F, F, F, F, F) {
    let t61165 = t2611 * t2609 * t5819;
    let t61180 = t4311 * t14440;
    let t61247 = t5941 * t123 * t2630;
    let t61282 = t18555 * t2619;
    let t61294 = t18562 * t2516;
    (t61165, t61180, t61247, t61282, t61294)
}
