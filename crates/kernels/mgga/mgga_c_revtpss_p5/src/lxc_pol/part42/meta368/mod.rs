//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1188;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1189;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1190;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1191;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1192;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1193;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1194;
use chunk7::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta368<F: Float>(t2411: F, t6079: F, t10446: F, t5819: F, t2375: F, t5825: F, t13309: F, t13310: F, t30: F, t33: F, zeta_threshold: F, t45: F, t57: F, t4186: F, t4377: F, t606: F, t78: F, t10457: F, t2382: F, t4384: F, t81: F, t150: F, t190: F, t5944: F, t750: F, t189: F, t4401: F, t10552: F, t10554: F, t14317: F, t18253: F, t18256: F, t18261: F, t18262: F, t18265: F, t18267: F, t1940: F, t2403: F, t4537: F, t4541: F, t4556: F, t775: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t1579: F, t4533: F, t2770: F, t212: F, t6041: F, t780: F, t689: F, t10498: F, t10501: F, t14474: F, t14479: F, t14484: F, t14486: F, t14985: F, t14989: F, t14992: F, t14995: F, t865: F, t6071: F, t886: F, t10673: F, t14675: F, t14690: F, t14703: F, t14705: F, t14712: F, t14715: F, t14716: F, t14722: F, t14726: F, t14730: F, t14734: F, t14494: F, t6035: F, t14791: F, t2703: F, t5985: F, t10905: F, t5989: F, t10678: F, t10687: F, t10692: F, t14736: F, t14744: F, t14759: F, t14761: F, t14765: F, t14777: F, t2745: F, t5962: F, t854: F, t236: F, t807: F, t2476: F, t5966: F, t10717: F, t10719: F, t10723: F, t10746: F, t10749: F, t14780: F, t14783: F, t14817: F, t14820: F, t14823: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18268, t18272, t18277, t18280) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1188::<F>(t2411, t6079, t10446, t5819, t2375, t5825, t13309, t13310);
        let t18281 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1189::<F>(t30, t33, t18280, zeta_threshold);
        let (t18285, t18297) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1190::<F>(t45, t57, t18272, t18277, t18281, t4186, t4377, t606, t78, t10457, t5819, t2382, t5825, t4384, t81, zeta_threshold);
        let (t18298, t18300, t18301, t18308, t18309) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1191::<F>(t18285, t18297, t150, t190, t5944, t750, t189, t5825, t606, t4401, t10552, t10554, t14317, t18253, t18256, t18261, t18262, t18265, t18267, t18268, t1940, t2403, t4537, t4541, t4556, t775, t9278, t9308, t9316, t9329, t9333);
        let t18322 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1192::<F>(t1579, t4533, t2770, t212, t6041, t780, t689, t10498, t10501, t14474, t14479, t14484, t14486, t14985, t14989, t14992, t14995, t865);
        let (t18324, t18330) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1193::<F>(t6071, t886, t2770, t10673, t14675, t14690, t14703, t14705, t14712, t14715, t14716, t14722, t14726, t14730, t14734);
        let t18343 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1194::<F>(t14494, t6035, t14791, t2703, t5985, t10905, t5989, t10678, t10687, t10692, t14736, t14744, t14759, t14761, t14765, t14777, t2745);
        let t18361 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1195::<F>(t5962, t854, t236, t807, t2476, t5966, t10717, t10719, t10723, t10746, t10749, t14780, t14783, t14817, t14820, t14823);
    (t18280, t18281, t18298, t18300, t18301, t18308, t18309, t18322, t18324, t18330, t18343, t18361)
}
