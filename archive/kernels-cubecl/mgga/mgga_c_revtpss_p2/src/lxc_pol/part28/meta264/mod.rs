//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta264 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1183;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1184;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1185;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1186;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta264<F: Float>(t1976: F, t988: F, t7145: F, t1981: F, t3056: F, t7143: F, t999: F, t1071: F, t1982: F, t3268: F, t359: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7146, t7147, t7150) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1183::<F>(t1976, t988, t7145, t1981, t3056);
        let t7151 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1184::<F>(t7143, t7150);
        let (t7152, t7153, t7156) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1185::<F>(t1976, t999, t7145, t1071, t1982);
        let t7159 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1186::<F>(t1982, t7143);
        let t7160 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1187::<F>(t3268, t359);
    (t7146, t7147, t7150, t7151, t7152, t7153, t7156, t7159, t7160)
}
