//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1789;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1790;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1791;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1792;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1793;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1794;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta493<F: Float>(t1544: F, t1583: F, t18875: F, t1940: F, t198: F, t207: F, t2071: F, t2403: F, t26585: F, t26590: F, t27375: F, t27384: F, t28455: F, t28460: F, t4343: F, t4433: F, t4537: F, t4541: F, t7428: F, t7432: F, t775: F, t8020: F, t890: F, t892: F, t30: F, t265: F, t393: F, t1469: F, t2078: F, t28491: F, t4186: F, t45: F, t606: F, t7449: F, t8040: F, t1113: F, t1711: F, t26425: F, t27764: F, t27770: F, t27773: F, t27777: F, t27793: F, t27800: F, t27802: F, t27806: F, t27810: F, t27817: F, t28291: F, t28456: F, t28472: F, t28490: F, t33: F, t7200: F, t7207: F, t7862: F, t7869: F, dens_threshold: F, rho0: F, zeta_threshold: F, t502: F, t2085: F, t57: F, t7468: F, t8059: F, t26405: F, t27153: F, rho1: F, t26179: F, t7706: F, t7349: F, t7709: F, t13272: F, t7342: F, t2048: F, t26180: F, t26185: F, t26187: F, t28105: F, t28109: F, t28112: F, t28116: F, t28119: F, t28141: F, t6960: F, t7343: F, t7352: F, t2047: F, t28150: F, t28089: F, t7702: F, t7348: F, t7719: F, t1923: F, t25162: F, t26170: F, t26175: F, t26182: F, t26190: F, t26207: F, t28093: F, t28133: F, t28147: F, t28154: F, t6954: F, t6963: F, t7964: F, t5: F, t117: F, t116: F, t7968: F, t2051: F, t670: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t28522 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1789::<F>(t1544, t1583, t18875, t1940, t198, t207, t2071, t2403, t26585, t26590, t27375, t27384, t28455, t28460, t4343, t4433, t4537, t4541, t7428, t7432, t775, t8020, t890, t892);
        let (t28523, t28530, t28577) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1790::<F>(t30, t265, t393, t28522, t1469, t2078, t28491, t4186, t45, t606, t7449, t8040, t1113, t1711, t1940, t2071, t2403, t26425, t26585, t27764, t27770, t27773, t27777, t27793, t27800, t27802, t27806, t27810, t27817, t28291, t28456, t28460, t28472, t28490, t33, t7200, t7207, t7428, t7432, t7862, t7869, t8020, dens_threshold, rho0, zeta_threshold);
        let (t28578, t28586, t28588) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1791::<F>(t33, t265, t502, t28522, t1469, t2085, t28577, t4186, t57, t606, t7468, t8059, t28530, t26405, t27153, dens_threshold, rho1, zeta_threshold);
        let (t28602, t28621) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1792::<F>(t26179, t7706, t7349, t7709, t13272, t7342, t2048, t26180, t26185, t26187, t28105, t28109, t28112, t28116, t28119, t28141, t6960, t7343, t7352);
        let (t28628, t28635, t28640, t28649) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1793::<F>(t2047, t28150, t28089, t7349, t7702, t7348, t7719, t1923, t2048, t25162, t26170, t26175, t26182, t26190, t26207, t28093, t28133, t28147, t28154, t6954, t6963, t7343, t7352, t7964);
        let (t28651, t28652, t28653) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1794::<F>(t5, t28621, t28649, t117, t116, t7968);
        let t28658 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1795::<F>(t2051, t670);
    (t28523, t28578, t28586, t28588, t28602, t28628, t28635, t28640, t28651, t28652, t28653, t28658)
}
