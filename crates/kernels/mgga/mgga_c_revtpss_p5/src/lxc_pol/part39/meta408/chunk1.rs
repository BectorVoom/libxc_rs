//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1485/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1485<F: Float>(t1459: F, t1461: F, t1916: F, t1918: F, t2187: F, t2189: F, t31340: F, t31359: F, t31362: F, t31365: F, t31371: F, t31374: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t8289: F, t8296: F, t8299: F, t8377: F, t8383: F, t8386: F) -> F {
    let t31377 = F::new(6.0) * t1459 * t8383 + F::new(3.0) * t1459 * t8386 + F::new(3.0) * t1461 * t8377 + F::new(6.0) * t1916 * t8296 + F::new(3.0) * t1916 * t8299 + F::new(3.0) * t1918 * t8289 + F::new(6.0) * t2187 * t5802 + F::new(3.0) * t2187 * t5805 + F::new(3.0) * t2189 * t5795 + t31340 * t573 + F::new(6.0) * t31359 * t572 + F::new(6.0) * t31362 * t572 + F::new(6.0) * t31365 * t572 + F::new(6.0) * t31371 * t572 + F::new(3.0) * t31374 * t572;
    t31377
}
