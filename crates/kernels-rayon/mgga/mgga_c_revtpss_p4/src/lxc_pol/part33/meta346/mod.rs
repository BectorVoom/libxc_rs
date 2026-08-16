//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1359;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1360;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta346(t1514: f64, t2289: f64, t4264: f64, t625: f64, t4288: f64, t2349: f64, t97: f64, t105: f64, t2357: f64, t1857: f64, t3857: f64, t177: f64, t5566: f64, t762: f64, t1450: f64, t5778: f64, t2516: f64, t5571: f64, t72: f64, t757: f64, t1320: f64, t5567: f64, t5569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13448, t13451, t13453, t13475, t13496, t13584, t13597) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1359(t1514, t2289, t4264, t625, t4288, t2349, t97, t105, t2357, t1857, t3857, t177, t5566);
        let (t13599, t13600, t13611, t13615, t13620, t13621) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1360(t13597, t762, t1450, t5778, t2516, t5571, t5566, t72, t757, t1320, t5567, t5569);
    (t13448, t13451, t13453, t13475, t13496, t13584, t13599, t13600, t13611, t13615, t13620, t13621)
}
