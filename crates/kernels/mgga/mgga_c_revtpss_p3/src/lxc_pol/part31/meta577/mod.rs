//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1995;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta577<F: Float>(t7064: F, t93150: F, t7015: F, t9292: F, t25411: F, t93183: F, t25387: F, t93285: F, t7063: F, t860: F, t25374: F, t11007: F, t1955: F, t7056: F) -> (F, F, F, F, F, F, F) {
        let (t93324, t93334, t93335, t93339, t93341, t93342, t93349) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1995::<F>(t7064, t93150, t7015, t9292, t25411, t93183, t25387, t93285, t7063, t860, t25374, t11007, t1955, t7056);
    (t93324, t93334, t93335, t93339, t93341, t93342, t93349)
}
