//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1760;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1761;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1762;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta371<F: Float>(t3434: F, t421: F, t12228: F, t12227: F, t1187: F, t3495: F, t3516: F, t1196: F, t1130: F, t3376: F, t1151: F, t3379: F, t3428: F, t1126: F, t3432: F, t3436: F, t3431: F, t418: F, t408: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12230, t12231, t12233, t12235, t12237, t12238, t12240, t12242) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1760::<F>(t3434, t421, t12228, t12227, t1187, t3495, t3516, t1196, t1130, t3376, t1151, t3379, t3428);
        let t12243 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1761::<F>(t1126, t3432);
        let (t12245, t12247, t12248) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1762::<F>(t12243, t3436, t3431, t418, t408);
    (t12230, t12231, t12233, t12235, t12237, t12238, t12240, t12242, t12243, t12245, t12247, t12248)
}
