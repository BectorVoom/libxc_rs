//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1791;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1792;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1793;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta490<F: Float>(t3336: F, t7177: F, t11108: F, t1989: F, t2411: F, t33: F, t116: F, t6982: F, t112: F, t239: F, t624: F, t655: F, t665: F, t2339: F, t68: F, t2033: F, t530: F, t555: F, t7063: F, t1032: F, t4075: F, t545: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25709, t25713, t25759) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1791::<F>(t3336, t7177, t11108, t1989, t2411, t33);
        let t25805 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1792::<F>(t116, t6982);
        let (t25822, t25823, t25824, t25826, t25864, t25875) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1793::<F>(t112, t239, t624, t655, t665, t2339, t68, t2033, t530, t555, t7063);
        let (t25876, t25877) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1794::<F>(t1032, t4075, t545);
    (t25709, t25713, t25759, t25805, t25822, t25823, t25824, t25826, t25864, t25875, t25876, t25877)
}
