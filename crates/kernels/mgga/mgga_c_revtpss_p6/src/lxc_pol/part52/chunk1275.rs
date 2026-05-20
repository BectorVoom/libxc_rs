//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1275/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1275<F: Float>(t34241: F, t531: F, t2014: F, t7238: F, t108120: F, t128477: F, t128917: F, t128920: F, t128930: F, t128932: F, t128933: F, t1310: F, t1911: F, t2056: F, t28030: F, t28653: F, t32660: F, t34188: F, t508: F, t5787: F, t7007: F, t7367: F, t8695: F, t97622: F) -> F {
    let t128934 = t531 * t34241;
    let t128937 = F::new(3.0) * t2014 * t128934 * t7238;
    let t128941 = -F::new(2.0) * t108120 * t2056 - t128477 * t508 - t1310 * t34188 + t1911 * t32660 - F::new(2.0) * t2056 * t97622 - F::new(2.0) * t28030 * t7367 - F::new(2.0) * t28653 * t7007 + t5787 * t8695 - t128917 - t128920 - t128930 - t128932 - t128933 + t128937;
    t128941
}
