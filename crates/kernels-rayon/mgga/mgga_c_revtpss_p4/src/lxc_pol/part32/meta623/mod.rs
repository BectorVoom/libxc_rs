//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1966;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta623(t29547: f64, t644: f64, t77: f64, t1927: f64, t5872: f64, t2247: f64, t5826: f64, t196: f64, t197: f64, t22525: f64, t1448: f64, t6781: f64, t1353: f64, t30122: f64, t1450: f64, t21969: f64, t1518: f64, t4245: f64, t1501: f64, t4292: f64, t21881: f64, t93: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108983, t108986, t108990, t109077, t109096) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1966(t29547, t644, t77, t1927, t5872, t2247, t5826, t196, t197, t22525, t1448, t6781);
        let (t109100, t109104, t109118, t109150, t109153, t109199, t109242) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1967(t1353, t6781, t30122, t1450, t21969, t1518, t4245, t1501, t4292, t1448, t21881, t93);
    (t108983, t108986, t108990, t109077, t109096, t109100, t109104, t109118, t109150, t109153, t109199, t109242)
}
