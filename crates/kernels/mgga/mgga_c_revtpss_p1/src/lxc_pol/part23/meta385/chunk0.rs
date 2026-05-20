//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1730/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1730<F: Float>(t16710: F, t16712: F, t1130: F, t5060: F, t1719: F, t3432: F) -> (F, F, F, F) {
    let t16821 = F::cast_from(0.12361111111111111111e-1_f64) * t16710;
    let t16822 = F::cast_from(0.61805555555555555556e-2_f64) * t16712;
    let t16835 = t5060 * t1130;
    let t16840 = t1719 * t3432;
    (t16821, t16822, t16835, t16840)
}
