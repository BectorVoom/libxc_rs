//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1735;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1736;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta389<F: Float>(t1770: F, t3781: F, t1284: F, t1811: F, t1209: F, t1263: F, t3362: F, t3172: F, t5298: F, t3711: F, t5278: F, t5269: F, t1261: F, t12256: F, t13099: F, t1224: F, t140: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t17183 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1735::<F>(t1770, t3781);
        let (t17191, t17192) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1736::<F>(t1284, t1811, t1209);
        let (t17202, t17209, t17211, t17217, t17219, t17225, t17227, t17235, t17240) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1737::<F>(t1263, t3362, t3172, t5298, t3711, t5278, t5269, t1261, t12256, t13099, t1224, t140);
    (t17183, t17191, t17192, t17202, t17209, t17211, t17217, t17219, t17225, t17227, t17235, t17240)
}
