//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 764/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk764<F: Float>(t239: F, t2247: F, t607: F, t1927: F, t644: F, t531: F, t7311: F, t1962: F, t198: F, t206: F, t2411: F, t30: F) -> (F, F, F, F, F, F) {
    let t25137 = 88.0 / 9.0 * t239;
    let t25162 = t2247 * t607;
    let t25163 = t1927 * t644;
    let t25190 = t531 * t7311;
    let t25206 = t198 * t206 * t1962;
    let t25207 = t2411 * t30;
    (t25137, t25162, t25163, t25190, t25206, t25207)
}
