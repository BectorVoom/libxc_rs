//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2280/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2280<F: Float>(t16831: F, t448: F, t300: F, t1130: F, t5060: F, t1151: F, t3428: F, t5063: F, t1719: F, t3432: F) -> (F, F, F, F, F, F) {
    let t16832 = t16831 * t448;
    let t16834 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t16832;
    let t16835 = t5060 * t1130;
    let t16837 = F::cast_from(2.0_f64) * t16835 * t1151;
    let t16839 = F::cast_from(1.0_f64) * t5063 * t3428;
    let t16840 = t1719 * t3432;
    (t16832, t16834, t16835, t16837, t16839, t16840)
}
