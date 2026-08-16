//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1872;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1873;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta517(t27679: f64, t7145: f64, t7828: f64, t999: f64, t7160: f64, t1651: f64, t7135: f64, t7821: f64, t1096: f64, t25464: f64, t1647: f64, t1976: f64, t7817: f64, t1097: f64, t1983: f64, t25473: f64, t25591: f64, t25605: f64, t25611: f64, t25629: f64, t25699: f64, t27653: f64, t27656: f64, t27661: f64, t27665: f64, t27669: f64, t27670: f64, t27676: f64, t7144: f64, t7147: f64, t7151: f64, t7159: f64, t7812: f64, t7829: f64, t989: f64, t27553: f64, t27592: f64, t27650: f64, t3336: f64, t7840: f64, t1100: f64, t1699: f64, t1544: f64, t1583: f64, t18875: f64, t1940: f64, t1963: f64, t198: f64, t207: f64, t2403: f64, t25440: f64, t25445: f64, t27363: f64, t27368: f64, t27375: f64, t27384: f64, t4343: f64, t4433: f64, t4537: f64, t4541: f64, t7087: f64, t7091: f64, t775: f64, t7783: f64, t890: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27680, t27684, t27687, t27688, t27692, t27696, t27699) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1872(t27679, t7145, t7828, t999, t7160, t1651, t7135, t7821, t1096, t25464, t1647, t1976);
        let (t27703, t27706) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1873(t7817, t999, t7145, t1097, t1983, t25473, t25591, t25605, t25611, t25629, t25699, t27653, t27656, t27661, t27665, t27669, t27670, t27676, t27680, t27684, t27688, t27692, t27696, t27699, t7144, t7147, t7151, t7159, t7812, t7829, t989);
        let (t27708, t27712, t27717, t27754) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1874(t27553, t27592, t27650, t27706, t3336, t7840, t1100, t1699, t1544, t1583, t18875, t1940, t1963, t198, t207, t2403, t25440, t25445, t27363, t27368, t27375, t27384, t4343, t4433, t4537, t4541, t7087, t7091, t775, t7783, t890, t892);
    (t27680, t27684, t27687, t27688, t27692, t27696, t27699, t27703, t27708, t27712, t27717, t27754)
}
