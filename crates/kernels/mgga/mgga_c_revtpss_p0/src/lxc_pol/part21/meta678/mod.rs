//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2489;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta678<F: Float>(t13100: F, t828: F, t12699: F, t3624: F, t12772: F, t12841: F, t5340: F, t12879: F, t3625: F, t3630: F, t1260: F, t12975: F, t1247: F, t1251: F, t42994: F, t1032: F, t1246: F, t12690: F, t12904: F, t3708: F, t11262: F, t3590: F, t3610: F, t3612: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44225, t44230, t44248, t44250, t44252, t44260) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2489::<F>(t13100, t828, t12699, t3624, t12772, t12841, t5340, t12879, t3625, t3630, t1260, t12975);
        let (t44264, t44267, t44270, t44273, t44276) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2490::<F>(t1247, t1251, t42994, t1032, t1246, t12690, t12904, t3708, t11262, t3590, t3610, t3612);
    (t44225, t44230, t44248, t44250, t44252, t44260, t44264, t44267, t44270, t44273, t44276)
}
