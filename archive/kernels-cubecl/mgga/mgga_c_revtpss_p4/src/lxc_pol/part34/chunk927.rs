//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 927/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk927<F: Float>(t1497: F, t5816: F, t5872: F, t1927: F, t5825: F, t1486: F, t5819: F, t22603: F) -> (F, F, F, F, F) {
    let t22656 = t5816 * t1497;
    let t22659 = t1497 * t5872;
    let t22662 = t1927 * t5825;
    let t22665 = t5819 * t1486;
    let t22670 = F::cast_from(6.0_f64) * t22603;
    (t22656, t22659, t22662, t22665, t22670)
}
