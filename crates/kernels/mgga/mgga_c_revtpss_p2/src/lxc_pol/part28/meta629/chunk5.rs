//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2270/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2270<F: Float>(t1518: F, t572: F, t670: F, t7002: F, t4158: F, t7953: F, t101469: F, t117: F, t2327: F, t7741: F, t101558: F, t101568: F, t101570: F, t101572: F, t101576: F, t101578: F, t101583: F, t101586: F, t101590: F, t18204: F, t18208: F, t18214: F, t1918: F, t2040: F, t26106: F, t573: F, param_d: F) -> F {
    let t101594 = F::new(12.0) * t572 * t670 * t7002 * t1518;
    let t101598 = F::new(3.0) * t4158 * t7953;
    let t101601 = F::new(3.0) * t572 * t117 * t101469;
    let t101606 = F::new(6.0) * t572 * t2327 * t7741;
    let t101609 = t101558 * t573 * param_d + F::new(6.0) * t18204 * t2040 + F::new(12.0) * t18208 * t2040 + F::new(3.0) * t18214 * t2040 + F::new(3.0) * t1918 * t26106 + t101568 + t101570 + t101572 + t101576 + t101578 + t101583 + t101586 + t101590 + t101594 + t101598 + t101601 + t101606;
    t101609
}
