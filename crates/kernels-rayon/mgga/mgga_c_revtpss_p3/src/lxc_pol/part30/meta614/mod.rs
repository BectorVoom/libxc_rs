//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2118;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2119;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta614(t25082: f64, t49582: f64, t8717: f64, t2014: f64, t25089: f64, t28172: f64, t27154: f64, t95088: f64, t26089: f64, t5542: f64, t2322: f64, t28043: f64, t4254: f64, t1310: f64, t28042: f64, t651: f64, t25851: f64, t4248: f64, t1518: f64, t2319: f64, t1937: f64, t4292: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98458, t98461, t98463, t98467, t98472) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2118(t25082, t49582, t8717, t2014, t25089, t28172, t27154, t95088, t26089, t5542, t2322, t28043);
        let (t98474, t98477, t98483, t98484, t98486, t98487) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2119(t28043, t4254, t1310, t28042, t651, t25851, t4248, t1518, t2319, t1937, t4292, t648);
    (t98458, t98461, t98463, t98467, t98472, t98474, t98477, t98483, t98484, t98486, t98487)
}
