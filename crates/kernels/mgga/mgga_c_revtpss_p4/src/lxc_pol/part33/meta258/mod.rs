//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta258 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1154;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1155;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1156;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1157;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta258<F: Float>(t1949: F, t886: F, t7071: F, t822: F, t867: F, t231: F, t836: F, t233: F, t7048: F, t1957: F, t1956: F, t1959: F, t213: F, t257: F, t7017: F, t7020: F, t7049: F, t7053: F, t7062: F, t7066: F, t7067: F, t7070: F, t887: F, t892: F, t1962: F, t2411: F, t30: F, t890: F, t1940: F, t1963: F, t2403: F, t605: F, t7010: F, t207: F, t198: F, t775: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7073, t7076) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1154::<F>(t1949, t886, t7071, t822, t867);
        let (t7078, t7079, t7082, t7083, t7086) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1155::<F>(t1949, t231, t836, t7076, t233, t7048, t1957, t1956, t1959, t213, t257, t7017, t7020, t7049, t7053, t7062, t7066, t7067, t7070, t7073, t887);
        let t7087 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1156::<F>(t7086, t892);
        let t7091 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1157::<F>(t1962, t2411);
        let (t7092, t7099, t7193) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1158::<F>(t30, t890, t1940, t1963, t2403, t605, t7010, t7087, t7091, t207, t7086, t198, t775, t892);
    (t7073, t7076, t7078, t7079, t7082, t7083, t7086, t7087, t7091, t7092, t7099, t7193)
}
