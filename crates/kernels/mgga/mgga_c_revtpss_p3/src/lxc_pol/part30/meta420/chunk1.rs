//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1585/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1585<F: Float>(t15691: F, t15692: F, t372: F, t4823: F, t3096: F, t1087: F, t11773: F, t4801: F, t4181: F, t4786: F, t1062: F, t4857: F) -> (F, F, F, F, F, F) {
    let t15693 = t15691 * t15692;
    let t15696 = t372 * t4823;
    let t15697 = t15696 * t3096;
    let t15700 = t1087 * t11773;
    let t15701 = t372 * t4801;
    let t15702 = t4181 * t4786;
    let t15703 = t15701 * t15702;
    let t15707 = t4857 * t1062;
    (t15693, t15697, t15700, t15702, t15703, t15707)
}
