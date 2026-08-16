//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1463;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1464;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1465;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1466;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1467;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1468;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta410(t2411: f64, t6079: f64, t10446: f64, t5819: f64, t2375: f64, t5825: f64, t13309: f64, t13310: f64, t30: f64, t33: f64, zeta_threshold: f64, t45: f64, t57: f64, t4186: f64, t4377: f64, t606: f64, t78: f64, t10457: f64, t2382: f64, t4384: f64, t81: f64, t150: f64, t190: f64, t5944: f64, t750: f64, t189: f64, t4401: f64, t10552: f64, t10554: f64, t14317: f64, t18253: f64, t18256: f64, t18261: f64, t18262: f64, t18265: f64, t18267: f64, t1940: f64, t2403: f64, t4537: f64, t4541: f64, t4556: f64, t775: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64, t1579: f64, t4533: f64, t2770: f64, t212: f64, t6041: f64, t780: f64, t689: f64, t10498: f64, t10501: f64, t14474: f64, t14479: f64, t14484: f64, t14486: f64, t14985: f64, t14989: f64, t14992: f64, t14995: f64, t865: f64, t6071: f64, t886: f64, t10673: f64, t14675: f64, t14690: f64, t14703: f64, t14705: f64, t14712: f64, t14715: f64, t14716: f64, t14722: f64, t14726: f64, t14730: f64, t14734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18268, t18272, t18277, t18280) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1463(t2411, t6079, t10446, t5819, t2375, t5825, t13309, t13310);
        let t18281 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1464(t30, t33, t18280, zeta_threshold);
        let (t18285, t18297) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1465(t45, t57, t18272, t18277, t18281, t4186, t4377, t606, t78, t10457, t5819, t2382, t5825, t4384, t81, zeta_threshold);
        let (t18298, t18300, t18301, t18308, t18309) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1466(t18285, t18297, t150, t190, t5944, t750, t189, t5825, t606, t4401, t10552, t10554, t14317, t18253, t18256, t18261, t18262, t18265, t18267, t18268, t1940, t2403, t4537, t4541, t4556, t775, t9278, t9308, t9316, t9329, t9333);
        let (t18313, t18322) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1467(t1579, t4533, t2770, t212, t6041, t780, t689, t10498, t10501, t14474, t14479, t14484, t14486, t14985, t14989, t14992, t14995, t865);
        let (t18324, t18330) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1468(t6071, t886, t2770, t10673, t14675, t14690, t14703, t14705, t14712, t14715, t14716, t14722, t14726, t14730, t14734);
    (t18280, t18281, t18298, t18300, t18301, t18308, t18309, t18313, t18322, t18324, t18330)
}
