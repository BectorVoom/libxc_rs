//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1738;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1739;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1740;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta366<F: Float>(t12051: F, t357: F, t12048: F, t1043: F, t1089: F, t3259: F, t12032: F, t380: F, t11620: F, t378: F, t359: F, t999: F, t11239: F, t3143: F, t342: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12052, t12053, t12057, t12066, t12070, t12073, t12074) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1738::<F>(t12051, t357, t12048, t1043, t1089, t3259, t12032, t380, t11620, t378, t359, t999);
        let t12077 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1739::<F>(t11239, t3143);
        let t12078 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1740::<F>(t12077, t342);
    (t12052, t12053, t12057, t12066, t12070, t12073, t12074, t12077, t12078)
}
