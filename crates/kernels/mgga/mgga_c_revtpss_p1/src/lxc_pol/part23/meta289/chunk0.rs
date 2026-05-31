//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1520/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1520<F: Float>(t3145: F, t334: F, t368: F, t365: F, t3144: F, t11240: F, t3153: F, t73: F) -> (F, F, F, F, F) {
    let t11243 = F::cast_from(1.0_f64) / t3145 / t368 / t334;
    let t11244 = t365 * t11243;
    let t11245 = t3144 * t11244;
    let t11246 = t11240 * t11245;
    let t11249 = t3153 * t73;
    (t11243, t11244, t11245, t11246, t11249)
}
