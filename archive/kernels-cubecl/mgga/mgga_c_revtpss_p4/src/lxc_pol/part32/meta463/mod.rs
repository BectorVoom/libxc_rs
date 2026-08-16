//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1687;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1688;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta463<F: Float>(t25304: F, t7283: F, t25949: F, t786: F, t1426: F, t3999: F, t25821: F, t2106: F, t530: F, t6977: F, t7348: F, t1923: F, t10309: F, t7342: F, t38: F, t624: F, t2247: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26069, t26072, t26079, t26148, t26161, t26169, t26170) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1687::<F>(t25304, t7283, t25949, t786, t1426, t3999, t25821, t2106, t530, t6977, t7348, t1923);
        let t26175 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1688::<F>(t10309, t7342);
        let (t26178, t26179) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1689::<F>(t38, t624, t2247);
    (t26069, t26072, t26079, t26148, t26161, t26169, t26170, t26175, t26178, t26179)
}
