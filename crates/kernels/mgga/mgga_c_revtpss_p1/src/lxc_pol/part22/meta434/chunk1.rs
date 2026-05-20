//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2062/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2062<F: Float>(t14749: F, t2477: F, t828: F, t14712: F, t14715: F, t14716: F, t14722: F, t14726: F, t14730: F, t14734: F, t14736: F, t14738: F, t14744: F, t14746: F, t799: F, t825: F, t851: F) -> (F, F) {
    let t14751 = t2477 * t828 * t14749;
    let t14754 = -F::cast_from(0.56688979511669985553e-2_f64) * t14712 + t14715 + F::cast_from(0.13552000749142754193e-3_f64) * t14716 - t14722 + t14726 - t14730 - t14734 - t14736 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t14738 + t14744 - t799 * t14746 / F::new(48.0) + F::cast_from(0.85748036236139473944e-2_f64) * t851 * t14751;
    (t14751, t14754)
}
