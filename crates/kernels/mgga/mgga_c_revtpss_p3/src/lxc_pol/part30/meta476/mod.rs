//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta476 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1798;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1799;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1800;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1801;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1802;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta476<F: Float>(t2411: F, t7086: F, t11064: F, t1962: F, t2408: F, t30: F, t605: F, t890: F, t2832: F, t1940: F, t1963: F, t2257: F, t2403: F, t25198: F, t25206: F, t25208: F, t25211: F, t25215: F, t25436: F, t4541: F, t7010: F, t7087: F, t7091: F, t7092: F, t14365: F, t198: F, t207: F, t2394: F, t2430: F, t25435: F, t775: F, t892: F, t33: F) -> (F, F, F, F, F, F, F, F, F) {
        let t25440 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1798::<F>(t2411, t7086);
        let t25445 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1799::<F>(t11064, t1962);
        let (t25446, t25449, t25452, t25459) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1800::<F>(t2408, t30, t605, t890, t2832, t1940, t1963, t2257, t2403, t25198, t25206, t25208, t25211, t25215, t25436, t25440, t25445, t4541, t7010, t7087, t7091, t7092);
        let t25743 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1801::<F>(t14365, t1940, t1963, t198, t207, t2394, t2403, t2408, t2430, t25435, t25440, t25445, t2832, t4541, t7087, t7091, t775, t890, t892);
        let (t25752, t25759) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1802::<F>(t2394, t33, t2411);
    (t25440, t25445, t25446, t25449, t25452, t25459, t25743, t25752, t25759)
}
