//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1103/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1103<F: Float>(t28025: F, t7735: F, t27137: F, t6985: F, t2322: F, t33574: F, t4254: F, t651: F, t7221: F, t7741: F, t25805: F, t7742: F) -> (F, F, F, F, F, F) {
    let t125541 = t28025 * t7735;
    let t125543 = t6985 * t27137;
    let t125545 = t2322 * t33574;
    let t125547 = t4254 * t33574;
    let t125550 = t651 * t7221 * t7741;
    let t125552 = t25805 * t7742;
    (t125541, t125543, t125545, t125547, t125550, t125552)
}
