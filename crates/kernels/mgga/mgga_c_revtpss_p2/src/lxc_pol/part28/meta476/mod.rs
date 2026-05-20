//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta476 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1803;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1804;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1805;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1806;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1807;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1808;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta476<F: Float>(t2411: F, t7086: F, t11064: F, t1962: F, t2408: F, t30: F, t605: F, t890: F, t2832: F, t1940: F, t1963: F, t2257: F, t2403: F, t25198: F, t25206: F, t25208: F, t25211: F, t25215: F, t25436: F, t4541: F, t7010: F, t7087: F, t7091: F, t7092: F, t1032: F, t1071: F, t7150: F, t11120: F, t359: F, t1976: F, t3270: F, t1096: F, t7135: F, t7160: F, t1982: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t25440 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1803::<F>(t2411, t7086);
        let t25445 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1804::<F>(t11064, t1962);
        let (t25446, t25449, t25452, t25459) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1805::<F>(t2408, t30, t605, t890, t2832, t1940, t1963, t2257, t2403, t25198, t25206, t25208, t25211, t25215, t25436, t25440, t25445, t4541, t7010, t7087, t7091, t7092);
        let t25460 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1806::<F>(t1032, t1071);
        let t25461 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1807::<F>(t25460, t7150);
        let t25464 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1808::<F>(t11120, t359);
        let (t25466, t25470, t25473) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1809::<F>(t1976, t3270, t25464, t1096, t7135, t7160, t1982, t25460);
    (t25440, t25445, t25446, t25449, t25452, t25459, t25460, t25461, t25464, t25466, t25470, t25473)
}
