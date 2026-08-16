//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta640(t1455: f64, t8249: f64, t116: f64, t29421: f64, t10301: f64, t29411: f64, t2247: f64, t29362: f64, t38: f64, t10309: f64, t60224: f64, t7565: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t104094, t104115, t104181, t104185, t104203, t104208) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2089(t1455, t8249, t116, t29421, t10301, t29411, t2247, t29362, t38, t10309, t60224, t7565);
    (t104094, t104115, t104181, t104185, t104203, t104208)
}
