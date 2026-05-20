//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1140/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1140<F: Float>(t28025: F, t7742: F, t28063: F, t6985: F, t27833: F, t8596: F, t1353: F, t7933: F, t25082: F, t8717: F, t1907: F, t7311: F) -> (F, F, F, F, F) {
    let t125554 = t28025 * t7742;
    let t125556 = t6985 * t28063;
    let t125558 = t27833 * t8596;
    let t125559 = t7933 * t1353;
    let t125562 = F::new(6.0) * t25082 * t8717 * t125559;
    let t125563 = t1907 * t7311;
    (t125554, t125556, t125558, t125562, t125563)
}
