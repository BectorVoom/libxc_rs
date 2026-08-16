//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1752;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1753;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1754;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1755;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1756;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1757;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1758;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta492(t2411: f64, t8019: f64, t198: f64, t2075: f64, t1940: f64, t2071: f64, t2255: f64, t1468: f64, t2403: f64, t26425: f64, t26585: f64, t27160: f64, t27166: f64, t27169: f64, t27173: f64, t27376: f64, t27385: f64, t27387: f64, t27391: f64, t27395: f64, t27402: f64, t28291: f64, t28456: f64, t30: f64, t605: f64, t7010: f64, t7092: f64, t7428: f64, t7432: f64, t7749: f64, t7787: f64, t8020: f64, t1544: f64, t1583: f64, t18875: f64, t207: f64, t26590: f64, t27375: f64, t27384: f64, t28455: f64, t4343: f64, t4433: f64, t4537: f64, t4541: f64, t775: f64, t890: f64, t892: f64, t265: f64, t393: f64, t1469: f64, t2078: f64, t4186: f64, t45: f64, t606: f64, t7449: f64, t8040: f64, t1113: f64, t1711: f64, t27764: f64, t27770: f64, t27773: f64, t27777: f64, t27793: f64, t27800: f64, t27802: f64, t27806: f64, t27810: f64, t27817: f64, t33: f64, t7200: f64, t7207: f64, t7862: f64, t7869: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t502: f64, t2085: f64, t57: f64, t7468: f64, t8059: f64, t26405: f64, t27153: f64, rho1: f64, t26179: f64, t7706: f64, t7349: f64, t7709: f64, t13272: f64, t7342: f64, t2048: f64, t26180: f64, t26185: f64, t26187: f64, t28105: f64, t28109: f64, t28112: f64, t28116: f64, t28119: f64, t28141: f64, t6960: f64, t7343: f64, t7352: f64, t2047: f64, t28150: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t28460 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1752(t2411, t8019);
        let t28472 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1753(t198, t2075);
        let (t28490, t28491) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1754(t1940, t2071, t2255, t1468, t2403, t26425, t26585, t27160, t27166, t27169, t27173, t27376, t27385, t27387, t27391, t27395, t27402, t28291, t28456, t28460, t28472, t30, t605, t7010, t7092, t7428, t7432, t7749, t7787, t8020);
        let t28522 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1755(t1544, t1583, t18875, t1940, t198, t207, t2071, t2403, t26585, t26590, t27375, t27384, t28455, t28460, t4343, t4433, t4537, t4541, t7428, t7432, t775, t8020, t890, t892);
        let (t28523, t28530, t28577) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1756(t30, t265, t393, t28522, t1469, t2078, t28491, t4186, t45, t606, t7449, t8040, t1113, t1711, t1940, t2071, t2403, t26425, t26585, t27764, t27770, t27773, t27777, t27793, t27800, t27802, t27806, t27810, t27817, t28291, t28456, t28460, t28472, t28490, t33, t7200, t7207, t7428, t7432, t7862, t7869, t8020, dens_threshold, rho0, zeta_threshold);
        let (t28578, t28586, t28588) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1757(t33, t265, t502, t28522, t1469, t2085, t28577, t4186, t57, t606, t7468, t8059, t28530, t26405, t27153, dens_threshold, rho1, zeta_threshold);
        let (t28598, t28600, t28602, t28621) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1758(t26179, t7706, t7349, t7709, t13272, t7342, t2048, t26180, t26185, t26187, t28105, t28109, t28112, t28116, t28119, t28141, t6960, t7343, t7352);
        let t28628 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1759(t2047, t28150);
    (t28460, t28472, t28523, t28578, t28586, t28588, t28598, t28600, t28602, t28621, t28628)
}
