//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta549<F: Float>(t1042: F, t17221: F, t3172: F, t5269: F, t1261: F, t13396: F, t5268: F, t12256: F, t13099: F, t15936: F, t1224: F, t140: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17222, t17225, t17227, t17231, t17232, t17235, t17236, t17237, t17240) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2223::<F>(t1042, t17221, t3172, t5269, t1261, t13396, t5268, t12256, t13099, t15936, t1224, t140);
    (t17222, t17225, t17227, t17231, t17232, t17235, t17236, t17237, t17240)
}
