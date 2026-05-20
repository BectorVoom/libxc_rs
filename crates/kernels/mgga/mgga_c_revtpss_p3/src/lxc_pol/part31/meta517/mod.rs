//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1872;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1873;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta517<F: Float>(t27679: F, t7145: F, t7828: F, t999: F, t7160: F, t1651: F, t7135: F, t7821: F, t1096: F, t25464: F, t1647: F, t1976: F, t7817: F, t1097: F, t1983: F, t25473: F, t25591: F, t25605: F, t25611: F, t25629: F, t25699: F, t27653: F, t27656: F, t27661: F, t27665: F, t27669: F, t27670: F, t27676: F, t7144: F, t7147: F, t7151: F, t7159: F, t7812: F, t7829: F, t989: F, t27553: F, t27592: F, t27650: F, t3336: F, t7840: F, t1100: F, t1699: F, t1544: F, t1583: F, t18875: F, t1940: F, t1963: F, t198: F, t207: F, t2403: F, t25440: F, t25445: F, t27363: F, t27368: F, t27375: F, t27384: F, t4343: F, t4433: F, t4537: F, t4541: F, t7087: F, t7091: F, t775: F, t7783: F, t890: F, t892: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27680, t27684, t27687, t27688, t27692, t27696, t27699) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1872::<F>(t27679, t7145, t7828, t999, t7160, t1651, t7135, t7821, t1096, t25464, t1647, t1976);
        let (t27703, t27706) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1873::<F>(t7817, t999, t7145, t1097, t1983, t25473, t25591, t25605, t25611, t25629, t25699, t27653, t27656, t27661, t27665, t27669, t27670, t27676, t27680, t27684, t27688, t27692, t27696, t27699, t7144, t7147, t7151, t7159, t7812, t7829, t989);
        let (t27708, t27712, t27717, t27754) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1874::<F>(t27553, t27592, t27650, t27706, t3336, t7840, t1100, t1699, t1544, t1583, t18875, t1940, t1963, t198, t207, t2403, t25440, t25445, t27363, t27368, t27375, t27384, t4343, t4433, t4537, t4541, t7087, t7091, t775, t7783, t890, t892);
    (t27680, t27684, t27687, t27688, t27692, t27696, t27699, t27703, t27708, t27712, t27717, t27754)
}
