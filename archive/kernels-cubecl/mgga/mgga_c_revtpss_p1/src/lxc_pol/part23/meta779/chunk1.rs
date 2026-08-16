//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2585/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2585<F: Float>(t13142: F, t56878: F, t12851: F, t1778: F, t3766: F, t5219: F, t5330: F, t3718: F, t44546: F, t5353: F, t45833: F, t58919: F) -> (F, F, F, F, F) {
    let t59066 = t13142 * t56878;
    let t59144 = t1778 * t12851;
    let t59162 = t5219 * t3766 * t5330;
    let t59185 = t3718 * t44546 * t5353;
    let t59186 = F::cast_from(0.14291339372689912324e-3_f64) * t59185;
    let t59196 = t45833 * t58919;
    (t59066, t59144, t59162, t59186, t59196)
}
