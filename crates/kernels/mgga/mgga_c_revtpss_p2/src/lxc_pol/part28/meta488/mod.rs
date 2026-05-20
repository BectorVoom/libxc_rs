//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1851;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1852;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta488<F: Float>(t1936: F, t3813: F, t651: F, t4254: F, t7003: F, t1310: F, t7002: F, t2033: F, t530: F, t1450: F, t3829: F, t2014: F, t670: F, t7221: F, t555: F, t7063: F, t1032: F, t4075: F, t545: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25856, t25858, t25860, t25861, t25863, t25865, t25866, t25868) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1851::<F>(t1936, t3813, t651, t4254, t7003, t1310, t7002, t2033, t530, t1450, t3829, t2014);
        let (t25872, t25875) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1852::<F>(t670, t7221, t555, t7063);
        let (t25876, t25877) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1853::<F>(t1032, t4075, t545);
    (t25856, t25858, t25860, t25861, t25863, t25865, t25866, t25868, t25872, t25875, t25876, t25877)
}
