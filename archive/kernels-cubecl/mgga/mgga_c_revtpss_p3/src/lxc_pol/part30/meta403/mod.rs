//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1510;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1511;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta403<F: Float>(t14723: F, t2662: F, t2661: F, t4416: F, t837: F, t221: F, t2485: F, t4424: F, t2484: F, t2652: F, t4435: F, t14663: F, t827: F, t828: F, t4343: F, t854: F, t236: F, t807: F, t124: F, t14468: F, t800: F, t775: F, t2477: F, t14712: F, t14715: F, t14716: F, t14722: F, t799: F, t825: F, t851: F) -> (F, F, F, F, F, F, F, F) {
        let (t14726, t14727, t14730, t14732, t14734, t14736, t14738) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1510::<F>(t14723, t2662, t2661, t4416, t837, t221, t2485, t4424, t2484, t2652, t4435, t14663, t827, t828);
        let (t14741, t14744, t14746, t14749) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1511::<F>(t4343, t854, t236, t807, t124, t14468, t800, t775);
        let (t14751, t14754) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1512::<F>(t14749, t2477, t828, t14712, t14715, t14716, t14722, t14726, t14730, t14734, t14736, t14738, t14744, t14746, t799, t825, t851);
    (t14727, t14732, t14738, t14741, t14746, t14749, t14751, t14754)
}
