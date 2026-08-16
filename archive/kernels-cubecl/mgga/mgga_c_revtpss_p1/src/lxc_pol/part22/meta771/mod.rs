//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta771 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2856;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta771<F: Float>(t13100: F, t828: F, t12699: F, t3624: F, t12879: F, t3625: F, t3630: F, t1260: F, t12975: F, t1247: F, t1251: F, t42994: F, t12904: F, t3708: F, t11262: F, t3590: F, t3610: F, t3612: F, t1231: F, t12898: F, t3651: F, t3655: F, t43813: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t44225, t44230, t44250, t44252, t44260, t44264) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2856::<F>(t13100, t828, t12699, t3624, t12879, t3625, t3630, t1260, t12975, t1247, t1251, t42994);
        let (t44270, t44273, t44276, t44291, t44293, t44307) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2857::<F>(t12904, t3708, t11262, t1247, t3590, t3610, t3612, t1231, t12898, t3651, t3655, t43813);
    (t44225, t44230, t44250, t44252, t44260, t44264, t44270, t44273, t44276, t44291, t44293, t44307)
}
