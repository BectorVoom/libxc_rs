//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta612<F: Float>(t1711: F, t4537: F, t25759: F, t77408: F, t6416: F, t890: F, t1113: F, t5966: F, t6075: F, t106610: F, t27799: F, t18435: F, t27763: F) -> (F, F, F, F, F, F, F) {
        let (t107988, t108002, t108005, t108009, t108021, t108028, t108030) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1951::<F>(t1711, t4537, t25759, t77408, t6416, t890, t1113, t5966, t6075, t106610, t27799, t18435, t27763);
    (t107988, t108002, t108005, t108009, t108021, t108028, t108030)
}
