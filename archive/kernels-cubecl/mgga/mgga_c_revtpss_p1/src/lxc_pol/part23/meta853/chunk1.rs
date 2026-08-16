//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2740/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2740<F: Float>(t12855: F, t12916: F, t20977: F, t20913: F, t3172: F, t3711: F, t21107: F, t3704: F, t17628: F, t5373: F, t20851: F, t3678: F) -> (F, F, F, F, F) {
    let t71630 = t12855 * t12916 * t20977;
    let t71687 = t3711 * t3172 * t20913;
    let t71710 = t21107 * t3704;
    let t71718 = t5373 * t17628;
    let t71738 = t20851 * t3678;
    (t71630, t71687, t71710, t71718, t71738)
}
