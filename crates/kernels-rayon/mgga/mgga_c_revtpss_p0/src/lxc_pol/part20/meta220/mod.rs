//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta220(t760: f64, t9419: f64, t2516: f64, t2523: f64, t9387: f64, t2496: f64, t189: f64, t606: f64, t2258: f64, t4401: f64, t9372: f64, t37: f64, t716: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10592, t10594, t10596, t10598, t10599, t10600, t10602, t10604, t10605) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1007(t760, t9419, t2516, t2523, t9387, t2496, t189, t606, t2258, t4401, t9372, t37, t716);
    (t10592, t10594, t10596, t10598, t10599, t10600, t10602, t10604, t10605)
}
