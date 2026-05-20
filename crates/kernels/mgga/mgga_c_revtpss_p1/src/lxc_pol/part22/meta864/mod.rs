//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta864 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3017;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta864<F: Float>(t40593: F, t4452: F, t10777: F, t14671: F, t14686: F, t2646: F, t4343: F, t836: F, t10943: F, t14931: F, t14933: F, t2482: F, t2668: F, t2719: F, t2710: F, t4371: F, t9732: F, t10886: F, t14833: F, t808: F, t10811: F, t14793: F, t14774: F, t2652: F, t10726: F, t14860: F, t2661: F, t4366: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t50634, t50643, t50649, t50673, t50681) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3017::<F>(t40593, t4452, t10777, t14671, t14686, t2646, t4343, t836, t10943, t14931, t14933, t2482, t2668, t2719);
        let (t50703, t50706, t50722, t50724, t50728) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3018::<F>(t2710, t4371, t9732, t10886, t14833, t808, t10811, t14793, t14774, t2652, t10726, t14860, t2661, t4366);
    (t50634, t50643, t50649, t50673, t50681, t50703, t50706, t50722, t50724, t50728)
}
