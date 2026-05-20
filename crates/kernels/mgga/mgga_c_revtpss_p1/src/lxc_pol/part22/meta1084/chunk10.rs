//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3935/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3935<F: Float>(t116: F, t117: F, t1459: F, t18190: F, t18208: F, t1916: F, t1918: F, t22556: F, t22559: F, t22568: F, t2327: F, t2371: F, t4162: F, t4165: F, t4292: F, t572: F, t573: F, t5883: F, t5920: F, t60595: F, t6941: F, t75494: F, t75657: F, t75716: F, param_d: F) -> F {
    let t75792 = F::new(12.0) * t116 * t572 * t75494 + F::new(3.0) * t117 * t572 * t75657 + F::new(6.0) * t2327 * t572 * t5920 + F::new(6.0) * t2371 * t572 * t5883 + F::new(24.0) * t4292 * t572 * t60595 + t573 * t75716 * param_d + F::new(12.0) * t1459 * t22556 + F::new(24.0) * t1459 * t22559 + F::new(6.0) * t1459 * t22568 + F::new(6.0) * t18190 * t1918 + F::new(24.0) * t18208 * t1916 + F::new(6.0) * t4162 * t6941 + F::new(3.0) * t4165 * t6941;
    t75792
}
