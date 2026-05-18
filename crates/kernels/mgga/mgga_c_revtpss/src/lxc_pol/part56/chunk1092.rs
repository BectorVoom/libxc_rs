//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1092/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1092<F: Float>(t13648: F, t2014: F, t8595: F, t33651: F, t7312: F, t28167: F, t37956: F, t5627: F, t27833: F, t8596: F, t1353: F, t7933: F) -> (F, F, F, F, F) {
    let t125525 = t2014 * t8595 * t13648;
    let t125531 = F::new(2.0) * t2014 * t7312 * t33651;
    let t125536 = F::new(6.0) * t28167 * t37956 * t5627;
    let t125558 = t27833 * t8596;
    let t125559 = t7933 * t1353;
    (t125525, t125531, t125536, t125558, t125559)
}
