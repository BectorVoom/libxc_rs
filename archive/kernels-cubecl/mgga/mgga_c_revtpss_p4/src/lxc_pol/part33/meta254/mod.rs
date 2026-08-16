//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1132;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1133;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1134;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta254<F: Float>(t624: F, t640: F, t76: F, t1937: F, t2322: F, t4254: F, t1310: F, t1936: F, t651: F, t112: F, t655: F, t68: F, t114: F, t665: F, t508: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6971, t6977, t6990, t6992, t6993) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1132::<F>(t624, t640, t76, t1937, t2322, t4254, t1310, t1936);
        let (t6995, t6997, t6998) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1133::<F>(t651, t6993, t112, t624, t655, t68);
        let t7002 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1134::<F>(t114, t665, t6998, t6997);
        let t7003 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1135::<F>(t508, t7002);
    (t6971, t6977, t6990, t6992, t6993, t6995, t6997, t6998, t7002, t7003)
}
