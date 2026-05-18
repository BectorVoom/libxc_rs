//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1420/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1420<F: Float>(t22564: F, t670: F, t117: F, t21881: F, t1459: F, t1461: F, t1916: F, t1918: F, t22544: F, t22556: F, t22559: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t6941: F, t6945: F, t6948: F) -> F {
    let t22565 = t22564 * t670;
    let t22568 = t117 * t21881;
    let t22571 = F::new(6.0) * t1459 * t6945 + F::new(3.0) * t1459 * t6948 + F::new(3.0) * t1461 * t6941 + F::new(12.0) * t1916 * t5802 + F::new(6.0) * t1916 * t5805 + F::new(6.0) * t1918 * t5795 + t22544 * t573 + F::new(6.0) * t22556 * t572 + F::new(12.0) * t22559 * t572 + F::new(6.0) * t22565 * t572 + F::new(3.0) * t22568 * t572;
    t22571
}
