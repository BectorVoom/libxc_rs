//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2476/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2476<F: Float>(t48299: F, t123: F, t2630: F, t5566: F, t13665: F, t9863: F, t9866: F, t47101: F, t9575: F, t9572: F, t1320: F, t13680: F) -> (F, F, F, F, F, F, F, F) {
    let t48300 = F::cast_from(0.51947577317044391276e2_f64) * t48299;
    let t48302 = t5566 * t123 * t2630;
    let t48303 = F::cast_from(0.32530743900905219526e-1_f64) * t48302;
    let t48304 = t13665 * t9863;
    let t48306 = t13665 * t9866;
    let t48312 = F::new(96.0) * t47101;
    let t48313 = t13665 * t9575;
    let t48324 = t13665 * t9572;
    let t48326 = t1320 * t13680;
    (t48300, t48303, t48304, t48306, t48312, t48313, t48324, t48326)
}
