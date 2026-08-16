//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta379(t1729: f64, t2439: f64, t5098: f64, t698: f64, t16708: f64, t16710: f64, t16712: f64, t5095: f64, t3523: f64, t5180: f64, t1737: f64, t3451: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16876, t16892, t16893, t16915, t16916, t16917, t16929, t16931, t16988, t17010, t17011, t17023) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1338(t1729, t2439, t5098, t698, t16708, t16710, t16712, t5095, t3523, t5180, t1737, t3451);
    (t16876, t16892, t16893, t16915, t16916, t16917, t16929, t16931, t16988, t17010, t17011, t17023)
}
