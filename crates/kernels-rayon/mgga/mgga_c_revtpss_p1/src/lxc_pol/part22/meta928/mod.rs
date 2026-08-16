//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta928 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3153;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta928(t12772: f64, t17639: f64, t3625: f64, t17645: f64, t1284: f64, t17288: f64, t3624: f64, t12917: f64, t17401: f64, t17396: f64, t1260: f64, t17289: f64, t17544: f64, t3708: f64, t12915: f64, t16771: f64, t247: f64, t5384: f64, t17763: f64, t3636: f64, t13085: f64, t5391: f64, t12881: f64, t5381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57026, t57029, t57040, t57045, t57049, t57053) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3153(t12772, t17639, t3625, t17645, t1284, t17288, t3624, t12917, t17401, t17396, t1260, t17289);
        let (t57063, t57070, t57075, t57077, t57094) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3154(t17544, t3708, t12915, t16771, t247, t5384, t17763, t3636, t13085, t5391, t12881, t5381);
    (t57026, t57029, t57040, t57045, t57049, t57053, t57063, t57070, t57075, t57077, t57094)
}
