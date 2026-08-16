//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1668;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1669;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1670;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1671;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1672;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1673;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta458<F: Float>(t112: F, t239: F, t624: F, t655: F, t665: F, t2339: F, t68: F, t555: F, t7063: F, t1032: F, t4075: F, t545: F, t786: F, t1385: F, t2028: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25821, t25823, t25825, t25826, t25875) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1668::<F>(t112, t239, t624, t655, t665, t2339, t68, t555, t7063);
        let (t25876, t25877) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1669::<F>(t1032, t4075, t545);
        let t25878 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1670::<F>(t25875, t25877);
        let t25894 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1671::<F>(t555, t786);
        let t25895 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1672::<F>(t25877, t25894);
        let t25898 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1673::<F>(t1385, t2028);
        let t25899 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1674::<F>(t25875, t25898);
    (t25821, t25823, t25825, t25826, t25875, t25876, t25877, t25878, t25894, t25895, t25898, t25899)
}
