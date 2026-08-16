//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta794 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2613;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2614;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta794(t18352: f64, t2710: f64, t2713: f64, t10722: f64, t6030: f64, t18419: f64, t9775: f64, t10777: f64, t18481: f64, t50945: f64, t18333: f64, t51123: f64, t18349: f64, t2689: f64, t14923: f64, t18521: f64, t124: f64, t5977: f64, t10779: f64, t2749: f64, t14686: f64, t14931: f64, t4366: f64, t2661: f64, t2662: f64, t61625: f64, t18599: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61888, t61890, t61892, t61913, t61916) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2613(t18352, t2710, t2713, t10722, t6030, t18419, t9775, t10777, t18481, t50945, t18333, t51123);
        let (t61924, t61952, t61956) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2614(t18349, t2689, t14923, t18521, t124, t5977);
        let (t61959, t61969, t61973, t61977) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2615(t10777, t10779, t2749, t61956, t14686, t14931, t4366, t2661, t2662, t61625, t18599, t837);
    (t61888, t61890, t61892, t61913, t61916, t61924, t61952, t61956, t61959, t61969, t61973, t61977)
}
