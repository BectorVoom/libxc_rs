//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1065/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1065<F: Float>(t30: F, t126434: F, t129301: F, t1469: F, t32785: F, t34388: F, t4186: F, t45: F, t606: F, t8752: F, t28184: F, t8764: F, t2322: F, t34428: F, t4254: F, t651: F, t7683: F, t7741: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t129308 = piecewise3(t120, t126434, t129301 * t45 / 2.0 + t32785 * t1469 / 2.0 + t34388 * t606 / 2.0 + t8752 * t4186 / 2.0);
    let t129312 = t8764 * t28184;
    let t129314 = t2322 * t34428;
    let t129316 = t4254 * t34428;
    let t129319 = t651 * t7683 * t7741;
    (t129308, t129312, t129314, t129316, t129319)
}
