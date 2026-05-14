//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 971/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk971<F: Float>(t33574: F, t4254: F, t651: F, t7221: F, t7741: F, t25805: F, t7742: F, t28025: F, t28063: F, t6985: F, t27833: F, t8596: F, t1353: F, t7933: F, t25082: F, t8717: F) -> (F, F, F, F, F, F, F) {
    let t125547 = t4254 * t33574;
    let t125550 = t651 * t7221 * t7741;
    let t125552 = t25805 * t7742;
    let t125554 = t28025 * t7742;
    let t125556 = t6985 * t28063;
    let t125558 = t27833 * t8596;
    let t125559 = t7933 * t1353;
    let t125562 = 6.0 * t25082 * t8717 * t125559;
    (t125547, t125550, t125552, t125554, t125556, t125558, t125562)
}
