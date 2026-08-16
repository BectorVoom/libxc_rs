//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta864 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3017;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta864(t40593: f64, t4452: f64, t10777: f64, t14671: f64, t14686: f64, t2646: f64, t4343: f64, t836: f64, t10943: f64, t14931: f64, t14933: f64, t2482: f64, t2668: f64, t2719: f64, t2710: f64, t4371: f64, t9732: f64, t10886: f64, t14833: f64, t808: f64, t10811: f64, t14793: f64, t14774: f64, t2652: f64, t10726: f64, t14860: f64, t2661: f64, t4366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50634, t50643, t50649, t50673, t50681) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3017(t40593, t4452, t10777, t14671, t14686, t2646, t4343, t836, t10943, t14931, t14933, t2482, t2668, t2719);
        let (t50703, t50706, t50722, t50724, t50728) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3018(t2710, t4371, t9732, t10886, t14833, t808, t10811, t14793, t14774, t2652, t10726, t14860, t2661, t4366);
    (t50634, t50643, t50649, t50673, t50681, t50703, t50706, t50722, t50724, t50728)
}
