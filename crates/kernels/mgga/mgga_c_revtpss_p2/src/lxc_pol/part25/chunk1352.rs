//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1352/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1352<F: Float>(t508: F, t651: F, t94991: F, t2014: F, t25177: F, t7312: F, t25178: F, t7235: F, t10416: F, t7003: F, t1937: F, t49693: F) -> (F, F, F, F, F) {
    let t95049 = F::new(2.0) * t651 * t508 * t94991;
    let t95056 = F::new(6.0) * t2014 * t7312 * t25177;
    let t95058 = F::new(6.0) * t7235 * t25178;
    let t95066 = F::new(6.0) * t10416 * t7003;
    let t95068 = F::new(6.0) * t49693 * t1937;
    (t95049, t95056, t95058, t95066, t95068)
}
