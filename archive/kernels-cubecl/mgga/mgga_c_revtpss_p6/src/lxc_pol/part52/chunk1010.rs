//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1010/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1010<F: Float>(t28986: F, t670: F, t117: F, t28683: F, t1459: F, t1461: F, t1916: F, t1918: F, t2113: F, t2115: F, t28956: F, t28975: F, t28978: F, t28981: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t7547: F, t7554: F, t7557: F, t8118: F, t8124: F, t8127: F) -> (F, F, F) {
    let t28987 = t28986 * t670;
    let t28990 = t117 * t28683;
    let t28993 = F::cast_from(6.0_f64) * t1459 * t8124 + F::cast_from(3.0_f64) * t1459 * t8127 + F::cast_from(3.0_f64) * t1461 * t8118 + F::cast_from(6.0_f64) * t1916 * t7554 + F::cast_from(3.0_f64) * t1916 * t7557 + F::cast_from(3.0_f64) * t1918 * t7547 + F::cast_from(6.0_f64) * t2113 * t5802 + F::cast_from(3.0_f64) * t2113 * t5805 + F::cast_from(3.0_f64) * t2115 * t5795 + t28956 * t573 + F::cast_from(6.0_f64) * t28975 * t572 + F::cast_from(6.0_f64) * t28978 * t572 + F::cast_from(6.0_f64) * t28981 * t572 + F::cast_from(6.0_f64) * t28987 * t572 + F::cast_from(3.0_f64) * t28990 * t572;
    (t28987, t28990, t28993)
}
