//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1171/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1171<F: Float>(t11941: F, t9770: F, t325: F, t33643: F, t11991: F, t11742: F, t129: F, t15805: F, t11741: F, t28370: F, t7200: F, t19055: F, t3284: F) -> (F, F, F, F, F, F) {
    let t33694 = t9770 * t11941;
    let t33696 = t325 * t33643;
    let t33697 = t33696 * t11991;
    let t33701 = t15805 * t129 * t11742;
    let t33704 = t11741 * t28370 * t7200;
    let t33707 = t11741 * t3284 * t19055;
    (t33694, t33696, t33697, t33701, t33704, t33707)
}
