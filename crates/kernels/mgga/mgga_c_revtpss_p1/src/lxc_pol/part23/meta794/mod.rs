//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta794 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2613;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2614;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta794<F: Float>(t18352: F, t2710: F, t2713: F, t10722: F, t6030: F, t18419: F, t9775: F, t10777: F, t18481: F, t50945: F, t18333: F, t51123: F, t18349: F, t2689: F, t14923: F, t18521: F, t124: F, t5977: F, t10779: F, t2749: F, t14686: F, t14931: F, t4366: F, t2661: F, t2662: F, t61625: F, t18599: F, t837: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t61888, t61890, t61892, t61913, t61916) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2613::<F>(t18352, t2710, t2713, t10722, t6030, t18419, t9775, t10777, t18481, t50945, t18333, t51123);
        let (t61924, t61952, t61956) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2614::<F>(t18349, t2689, t14923, t18521, t124, t5977);
        let (t61959, t61969, t61973, t61977) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2615::<F>(t10777, t10779, t2749, t61956, t14686, t14931, t4366, t2661, t2662, t61625, t18599, t837);
    (t61888, t61890, t61892, t61913, t61916, t61924, t61952, t61956, t61959, t61969, t61973, t61977)
}
