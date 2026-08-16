//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1944;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1945;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1946;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta522(t25464: f64, t27695: f64, t1647: f64, t1976: f64, t7817: f64, t999: f64, t7145: f64, t1097: f64, t1983: f64, t25473: f64, t25591: f64, t25605: f64, t25611: f64, t25629: f64, t25699: f64, t27653: f64, t27656: f64, t27661: f64, t27665: f64, t27669: f64, t27670: f64, t27676: f64, t27680: f64, t27684: f64, t27688: f64, t27692: f64, t7144: f64, t7147: f64, t7151: f64, t7159: f64, t7812: f64, t7829: f64, t989: f64, t27553: f64, t27592: f64, t27650: f64, t3336: f64, t7840: f64, t1100: f64, t1699: f64, t1544: f64, t1583: f64, t18875: f64, t1940: f64, t1963: f64, t198: f64, t207: f64, t2403: f64, t25440: f64, t25445: f64, t27363: f64, t27368: f64, t27375: f64, t27384: f64, t4343: f64, t4433: f64, t4537: f64, t4541: f64, t7087: f64, t7091: f64, t775: f64, t7783: f64, t890: f64, t892: f64, t265: f64, t393: f64, t1102: f64, t25709: f64, t25713: f64, t336: f64, t5019: f64, t5023: f64, t7181: f64, t30: f64, t1469: f64, t1996: f64, t27408: f64, t4186: f64, t45: f64, t606: f64, t7194: f64, t7856: f64, t33: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27696, t27699, t27702, t27703, t27706) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1944(t25464, t27695, t1647, t1976, t7817, t999, t7145, t1097, t1983, t25473, t25591, t25605, t25611, t25629, t25699, t27653, t27656, t27661, t27665, t27669, t27670, t27676, t27680, t27684, t27688, t27692, t7144, t7147, t7151, t7159, t7812, t7829, t989);
        let (t27708, t27712, t27717, t27754) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1945(t27553, t27592, t27650, t27706, t3336, t7840, t1100, t1699, t1544, t1583, t18875, t1940, t1963, t198, t207, t2403, t25440, t25445, t27363, t27368, t27375, t27384, t4343, t4433, t4537, t4541, t7087, t7091, t775, t7783, t890, t892);
        let t27755 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1946(t265, t393, t1100, t1102, t1699, t198, t25709, t25713, t27708, t27712, t27717, t27754, t336, t5019, t5023, t7181);
        let (t27762, t27763, t27764) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1947(t30, t1469, t1996, t27408, t27755, t4186, t45, t606, t7194, t7856, t33, t892, t4433, dens_threshold, rho0, zeta_threshold);
    (t27696, t27699, t27702, t27703, t27708, t27712, t27717, t27754, t27755, t27762, t27763, t27764)
}
