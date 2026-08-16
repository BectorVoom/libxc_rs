//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2156;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2157;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2158;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta662(t25759: f64, t77408: f64, t6416: f64, t890: f64, t1113: f64, t5966: f64, t6075: f64, t106610: f64, t27799: f64, t18435: f64, t27763: f64, t18498: f64, t106554: f64, t18838: f64, t33: f64, t106482: f64, t106516: f64, t1711: f64, t1940: f64, t1963: f64, t2403: f64, t27158: f64, t27364: f64, t27368: f64, t27382: f64, t27810: f64, t27817: f64, t29964: f64, t4541: f64, t7091: f64, t7207: f64, t7783: f64, t93404: f64, t107922: f64, t107963: f64, t108001: f64, t22279: f64, t28167: f64, t8996: f64, t29506: f64, t7313: f64, t1843: f64, t28042: f64, t651: f64, t2322: f64, t30005: f64, t4254: f64, t1310: f64, t30004: f64, t27123: f64, t7742: f64, t27126: f64, t28063: f64, t7732: f64, t28056: f64, t4248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108002, t108005, t108009, t108021, t108028, t108030, t108033) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2156(t25759, t77408, t6416, t890, t1113, t5966, t6075, t106610, t27799, t18435, t27763, t18498);
        let t108047 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2157(t106554, t27799, t18838, t33, t106482, t106516, t108002, t108005, t108009, t108021, t108028, t108030, t108033, t1711, t1940, t1963, t2403, t27158, t27364, t27368, t27382, t27810, t27817, t29964, t4541, t7091, t7207, t7783, t93404);
        let (t108049, t108067, t108068, t108076) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2158(t107922, t107963, t108001, t108047, t22279, t28167, t8996, t29506, t7313, t1843, t28042, t651);
        let (t108078, t108080, t108083, t108085, t108087, t108089, t108099) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2159(t2322, t30005, t4254, t1310, t30004, t651, t27123, t7742, t27126, t28063, t7732, t28056, t4248);
    (t108049, t108067, t108068, t108076, t108078, t108080, t108083, t108085, t108087, t108089, t108099)
}
