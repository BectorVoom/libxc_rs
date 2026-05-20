//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2197;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta670<F: Float>(t29547: F, t644: F, t77: F, t1927: F, t5872: F, t2247: F, t5826: F, t27154: F, t98450: F, t28177: F, t7898: F, t28043: F, t4248: F, t651: F, t6765: F, t7002: F, t28167: F, t86753: F, t8717: F, t13648: F, t2014: F, t7934: F, t29589: F, t7235: F, t13426: F, t7742: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t108983, t108986, t108990, t109012, t109014, t109024) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2197::<F>(t29547, t644, t77, t1927, t5872, t2247, t5826, t27154, t98450, t28177, t7898, t28043, t4248);
        let (t109029, t109035, t109038, t109039, t109041) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2198::<F>(t651, t6765, t7002, t28167, t86753, t8717, t13648, t2014, t7934, t29589, t7235, t13426, t7742);
    (t108983, t108986, t108990, t109012, t109014, t109024, t109029, t109035, t109038, t109039, t109041)
}
