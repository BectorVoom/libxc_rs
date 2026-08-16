//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta821 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2935;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta821(t22: f64, t46389: f64, t543: f64, t5735: f64, t1432: f64, t5763: f64, t9288: f64, t1892: f64, t3923: f64, t2782: f64, t4003: f64, t5744: f64, t10069: f64, t14124: f64, t14129: f64, t14231: f64, t10014: f64, t14216: f64, t13921: f64, t4101: f64, t686: f64, t72: f64, t10139: f64, t136: f64, t2457: f64, t5659: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47967, t47971, t47973, t47976) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2935(t22, t46389, t543, t5735, t1432, t5763, t9288, t1892, t3923, t2782, t4003, t5744);
        let (t47978, t47980, t47985, t47995, t47999, t48003) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2936(t10069, t14124, t14129, t14231, t10014, t14216, t13921, t4101, t686, t72, t10139, t136, t2457, t5659);
    (t47967, t47971, t47973, t47976, t47978, t47980, t47985, t47995, t47999, t48003)
}
