//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2384;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2385;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta624<F: Float>(t2661: F, t2662: F, t2749: F, t40378: F, t2430: F, t853: F, t837: F, t836: F, t124: F, t2645: F, t14686: F, t14931: F, t4366: F, t2722: F, t10777: F, t10779: F, t2682: F, t820: F, t823: F, t2751: F, t10886: F, t808: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40553, t40555, t40558, t40560, t40578, t40581) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2384::<F>(t2661, t2662, t2749, t40378, t2430, t853, t837, t836, t124, t2645, t14686, t14931, t4366);
        let (t40583, t40586, t40593, t40594, t40600) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2385::<F>(t124, t2722, t10777, t10779, t2749, t2682, t820, t823, t2751, t10886, t40555, t808);
    (t40553, t40558, t40560, t40578, t40581, t40583, t40586, t40593, t40594, t40600)
}
