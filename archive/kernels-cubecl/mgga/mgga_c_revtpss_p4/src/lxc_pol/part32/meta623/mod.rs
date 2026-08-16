//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1966;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta623<F: Float>(t29547: F, t644: F, t77: F, t1927: F, t5872: F, t2247: F, t5826: F, t196: F, t197: F, t22525: F, t1448: F, t6781: F, t1353: F, t30122: F, t1450: F, t21969: F, t1518: F, t4245: F, t1501: F, t4292: F, t21881: F, t93: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t108983, t108986, t108990, t109077, t109096) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1966::<F>(t29547, t644, t77, t1927, t5872, t2247, t5826, t196, t197, t22525, t1448, t6781);
        let (t109100, t109104, t109118, t109150, t109153, t109199, t109242) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1967::<F>(t1353, t6781, t30122, t1450, t21969, t1518, t4245, t1501, t4292, t1448, t21881, t93);
    (t108983, t108986, t108990, t109077, t109096, t109100, t109104, t109118, t109150, t109153, t109199, t109242)
}
