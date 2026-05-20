//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1400/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1400<F: Float>(t11032: F, t786: F, t789: F, t2453: F, t2458: F, t2761: F, t2444: F, t2772: F, t689: F, t11029: F, t9303: F, t39501: F, t781: F) -> (F, F, F, F, F) {
    let t41026 = t786 * t11032 * t789;
    let t41029 = t2453 * t2761 * t2458;
    let t41032 = t689 * t2444 * t2772;
    let t41034 = t9303 * t11029;
    let t41037 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t781;
    (t41026, t41029, t41032, t41034, t41037)
}
