//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2156;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2157;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2158;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta662<F: Float>(t25759: F, t77408: F, t6416: F, t890: F, t1113: F, t5966: F, t6075: F, t106610: F, t27799: F, t18435: F, t27763: F, t18498: F, t106554: F, t18838: F, t33: F, t106482: F, t106516: F, t1711: F, t1940: F, t1963: F, t2403: F, t27158: F, t27364: F, t27368: F, t27382: F, t27810: F, t27817: F, t29964: F, t4541: F, t7091: F, t7207: F, t7783: F, t93404: F, t107922: F, t107963: F, t108001: F, t22279: F, t28167: F, t8996: F, t29506: F, t7313: F, t1843: F, t28042: F, t651: F, t2322: F, t30005: F, t4254: F, t1310: F, t30004: F, t27123: F, t7742: F, t27126: F, t28063: F, t7732: F, t28056: F, t4248: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t108002, t108005, t108009, t108021, t108028, t108030, t108033) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2156::<F>(t25759, t77408, t6416, t890, t1113, t5966, t6075, t106610, t27799, t18435, t27763, t18498);
        let t108047 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2157::<F>(t106554, t27799, t18838, t33, t106482, t106516, t108002, t108005, t108009, t108021, t108028, t108030, t108033, t1711, t1940, t1963, t2403, t27158, t27364, t27368, t27382, t27810, t27817, t29964, t4541, t7091, t7207, t7783, t93404);
        let (t108049, t108067, t108068, t108076) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2158::<F>(t107922, t107963, t108001, t108047, t22279, t28167, t8996, t29506, t7313, t1843, t28042, t651);
        let (t108078, t108080, t108083, t108085, t108087, t108089, t108099) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2159::<F>(t2322, t30005, t4254, t1310, t30004, t651, t27123, t7742, t27126, t28063, t7732, t28056, t4248);
    (t108049, t108067, t108068, t108076, t108078, t108080, t108083, t108085, t108087, t108089, t108099)
}
