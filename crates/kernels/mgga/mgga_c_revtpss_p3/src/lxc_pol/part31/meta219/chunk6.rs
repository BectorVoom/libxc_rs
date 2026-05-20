//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 986/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk986<F: Float>(t116: F, t1518: F, t670: F, t117: F, t4292: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, t5795: F) -> (F, F, F, F) {
    let t5801 = t116 * t1518;
    let t5802 = t5801 * t670;
    let t5805 = t117 * t4292;
    let t5808 = F::new(3.0) * t1459 * t1918 + F::new(3.0) * t1461 * t1916 + F::new(6.0) * t572 * t5802 + F::new(3.0) * t572 * t5805 + t573 * t5795;
    (t5801, t5802, t5805, t5808)
}
