//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1027;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1028;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta259<F: Float>(t1284: F, t1811: F, t1209: F, t1263: F, t3362: F, t12256: F, t13099: F, t1224: F, t140: F, t1789: F, t371: F, t676: F, t1235: F, t1769: F, t3565: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17191, t17192, t17202, t17235, t17240, t17303) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1027::<F>(t1284, t1811, t1209, t1263, t3362, t12256, t13099, t1224, t140, t1789, t371, t676);
        let (t17304, t17306) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1028::<F>(t1235, t17303, t1769, t3565);
        let t17307 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1029::<F>(t17306, t225);
    (t17191, t17192, t17202, t17235, t17240, t17303, t17304, t17306, t17307)
}
