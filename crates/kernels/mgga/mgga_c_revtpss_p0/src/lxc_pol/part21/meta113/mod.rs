//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk735;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk736;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk737;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk738;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta113<F: Float>(t2722: F, t2723: F, t827: F, t828: F, t159: F, t243: F, t216: F, t124: F, t2394: F, t800: F, t2712: F, t785: F, t225: F, t826: F, t849: F, t820: F, t823: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2724 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk735::<F>(t2722, t2723);
        let (t2726, t2729, t2730) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk736::<F>(t2724, t827, t828, t159, t243, t216);
        let (t2731, t2732, t2735) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk737::<F>(t124, t2394, t800, t2712, t785);
        let (t2736, t2737, t2739, t2741) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk738::<F>(t225, t2735, t826, t849, t820, t823, t843);
    (t2724, t2726, t2729, t2730, t2731, t2732, t2735, t2736, t2737, t2739, t2741)
}
