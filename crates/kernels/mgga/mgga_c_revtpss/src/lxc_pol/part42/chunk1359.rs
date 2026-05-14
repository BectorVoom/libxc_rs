//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1359/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1359<F: Float>(t118822: F, t118864: F, t118911: F, t118955: F, t117: F, t118630: F, t118749: F, t1459: F, t1461: F, t1916: F, t1918: F, t2207: F, t2209: F, t22544: F, t22556: F, t31475: F, t31494: F, t31497: F, t31500: F, t31711: F, t31728: F, t35858: F, t4292: F, t572: F, t573: F, t5795: F, t5805: F, t5883: F, t670: F, t6941: F, t6948: F, t8320: F, t8336: F, t8343: F, t8421: F, t8427: F, t8430: F) -> (F, F) {
    let t118957 = t118822 + t118864 + t118911 + t118955;
    let t118962 = 6.0 * t572 * t118630 * t670 + 12.0 * t1916 * t31500 + 6.0 * t2207 * t22556 + 3.0 * t572 * t117 * t118749 + 3.0 * t31711 * t1461 + 12.0 * t1916 * t31497 + 3.0 * t8336 * t6948 + 6.0 * t8421 * t5805 + 12.0 * t1916 * t31494 + 6.0 * t6941 * t8343 + 6.0 * t31475 * t1918 + 6.0 * t5795 * t8430 + 6.0 * t572 * t5883 * t8320 + 12.0 * t1459 * t31728 + 3.0 * t22544 * t2209 + 12.0 * t572 * t35858 * t4292 + param_d * t118957 * t573 + 12.0 * t5795 * t8427;
    (t118957, t118962)
}
