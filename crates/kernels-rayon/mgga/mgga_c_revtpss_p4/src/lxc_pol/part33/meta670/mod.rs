//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2197;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta670(t29547: f64, t644: f64, t77: f64, t1927: f64, t5872: f64, t2247: f64, t5826: f64, t27154: f64, t98450: f64, t28177: f64, t7898: f64, t28043: f64, t4248: f64, t651: f64, t6765: f64, t7002: f64, t28167: f64, t86753: f64, t8717: f64, t13648: f64, t2014: f64, t7934: f64, t29589: f64, t7235: f64, t13426: f64, t7742: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108983, t108986, t108990, t109012, t109014, t109024) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2197(t29547, t644, t77, t1927, t5872, t2247, t5826, t27154, t98450, t28177, t7898, t28043, t4248);
        let (t109029, t109035, t109038, t109039, t109041) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2198(t651, t6765, t7002, t28167, t86753, t8717, t13648, t2014, t7934, t29589, t7235, t13426, t7742);
    (t108983, t108986, t108990, t109012, t109014, t109024, t109029, t109035, t109038, t109039, t109041)
}
