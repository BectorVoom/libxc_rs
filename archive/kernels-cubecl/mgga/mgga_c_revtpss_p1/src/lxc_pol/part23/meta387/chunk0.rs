//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1733/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1733<F: Float>(t16710: F, t16712: F, t1737: F, t3451: F, t1160: F, t5117: F, t3476: F) -> (F, F, F, F, F) {
    let t17010 = F::cast_from(0.2283111111111111111e-1_f64) * t16710;
    let t17011 = F::cast_from(0.11415555555555555555e-1_f64) * t16712;
    let t17023 = t1737 * t3451;
    let t17026 = t5117 * t1160;
    let t17032 = t1737 * t3476;
    (t17010, t17011, t17023, t17026, t17032)
}
