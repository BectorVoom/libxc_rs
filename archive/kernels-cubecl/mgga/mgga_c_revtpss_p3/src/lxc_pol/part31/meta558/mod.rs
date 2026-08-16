//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta558<F: Float>(t4147: F, t7311: F, t1925: F, t36: F, t1353: F, t2033: F, t1518: F, t1931: F, t7933: F, t1469: F, t1450: F, t11239: F, t3268: F) -> (F, F, F, F, F, F, F) {
        let (t32113, t32737, t33602, t33651, t34176, t35669, t36870) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1968::<F>(t4147, t7311, t1925, t36, t1353, t2033, t1518, t1931, t7933, t1469, t1450, t11239, t3268);
    (t32113, t32737, t33602, t33651, t34176, t35669, t36870)
}
