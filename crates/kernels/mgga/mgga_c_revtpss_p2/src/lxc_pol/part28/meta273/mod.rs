//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1222;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1223;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1224;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1225;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta273<F: Float>(t114: F, t651: F, t7735: F, t1513: F, t6998: F, t6997: F, t508: F, t1518: F, t2007: F, t1544: F, t30: F, t1963: F, t1549: F, t7025: F, t1561: F, t7038: F, t1565: F, t7045: F, t7024: F, t7032: F, t7035: F, t7042: F, t225: F, t1568: F, t1955: F, t1579: F, t1949: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7737, t7741) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1222::<F>(t114, t651, t7735, t1513, t6998, t6997);
        let t7742 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1223::<F>(t508, t7741);
        let (t7744, t7746) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1224::<F>(t651, t7742, t1518, t2007);
        let (t7749, t7750, t7759) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1225::<F>(t1544, t30, t1963, t1549, t7025, t1561, t7038, t1565, t7045, t7024, t7032, t7035, t7042);
        let (t7760, t7766, t7769) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1226::<F>(t225, t7759, t1568, t1955, t1579, t1949);
    (t7737, t7741, t7742, t7744, t7746, t7749, t7750, t7759, t7760, t7766, t7769)
}
