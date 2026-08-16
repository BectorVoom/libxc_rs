//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta530(t16668: f64, t3385: f64, t12227: f64, t3520: f64, t5180: f64, t5206: f64, t1196: f64, t3495: f64, t1189: f64, t3543: f64, t5192: f64, t3516: f64, t5197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16669, t16671, t16672, t16673, t16675, t16676, t16677, t16679, t16681, t16682) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2175(t16668, t3385, t12227, t3520, t5180, t5206, t1196, t3495, t1189, t3543, t5192, t3516, t5197);
    (t16669, t16671, t16672, t16673, t16675, t16676, t16677, t16679, t16681, t16682)
}
