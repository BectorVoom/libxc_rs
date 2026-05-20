//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1786/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1786<F: Float>(t28986: F, t670: F, t117: F, t28683: F, t1459: F, t1461: F, t1916: F, t1918: F, t2113: F, t2115: F, t28956: F, t28975: F, t28978: F, t28981: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t7547: F, t7554: F, t7557: F, t8118: F, t8124: F, t8127: F) -> (F, F, F) {
    let t28987 = t28986 * t670;
    let t28990 = t117 * t28683;
    let t28993 = F::new(6.0) * t1459 * t8124 + F::new(3.0) * t1459 * t8127 + F::new(3.0) * t1461 * t8118 + F::new(6.0) * t1916 * t7554 + F::new(3.0) * t1916 * t7557 + F::new(3.0) * t1918 * t7547 + F::new(6.0) * t2113 * t5802 + F::new(3.0) * t2113 * t5805 + F::new(3.0) * t2115 * t5795 + t28956 * t573 + F::new(6.0) * t28975 * t572 + F::new(6.0) * t28978 * t572 + F::new(6.0) * t28981 * t572 + F::new(6.0) * t28987 * t572 + F::new(3.0) * t28990 * t572;
    (t28987, t28990, t28993)
}
