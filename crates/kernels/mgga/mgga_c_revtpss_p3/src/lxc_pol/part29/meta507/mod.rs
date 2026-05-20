//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1825;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta507<F: Float>(t4343: F, t890: F, t1544: F, t2408: F, t4537: F, t775: F, t2832: F, t2411: F, t14365: F, t1448: F, t5591: F, t1868: F, t4144: F) -> (F, F, F, F, F, F, F, F) {
        let (t61102, t61155, t61182, t61203, t63164, t63186, t73394, t73488) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1825::<F>(t4343, t890, t1544, t2408, t4537, t775, t2832, t2411, t14365, t1448, t5591, t1868, t4144);
    (t61102, t61155, t61182, t61203, t63164, t63186, t73394, t73488)
}
