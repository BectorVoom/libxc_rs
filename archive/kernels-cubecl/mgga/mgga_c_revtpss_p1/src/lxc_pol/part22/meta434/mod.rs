//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2061;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta434<F: Float>(t4343: F, t854: F, t236: F, t807: F, t124: F, t14468: F, t800: F, t775: F, t2477: F, t828: F, t14712: F, t14715: F, t14716: F, t14722: F, t14726: F, t14730: F, t14734: F, t14736: F, t14738: F, t799: F, t825: F, t851: F) -> (F, F, F, F, F, F, F) {
        let (t14741, t14742, t14744, t14746, t14749) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2061::<F>(t4343, t854, t236, t807, t124, t14468, t800, t775);
        let (t14751, t14754) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2062::<F>(t14749, t2477, t828, t14712, t14715, t14716, t14722, t14726, t14730, t14734, t14736, t14738, t14744, t14746, t799, t825, t851);
    (t14741, t14742, t14744, t14746, t14749, t14751, t14754)
}
