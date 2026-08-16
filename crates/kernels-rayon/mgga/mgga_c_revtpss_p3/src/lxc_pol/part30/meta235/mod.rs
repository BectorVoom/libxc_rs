//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta235(t1247: f64, t5265: f64, t1263: f64, t3367: f64, t4181: f64, t1042: f64, t1032: f64, t1770: f64, t1246: f64, t1774: f64, t1122: f64, t5062: f64, t5065: f64, t5067: f64, t5070: f64, t5107: f64, t5111: f64, t5189: f64, t5191: f64, t5194: f64, t5196: f64, t5200: f64, t5204: f64, t5209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5266, t5268, t5269, t5270, t5273, t5274, t5277, t5278, t5279, t5284) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1069(t1247, t5265, t1263, t3367, t4181, t1042, t1032, t1770, t1246, t1774, t1122, t5062, t5065, t5067, t5070, t5107, t5111, t5189, t5191, t5194, t5196, t5200, t5204, t5209);
    (t5266, t5268, t5269, t5270, t5273, t5274, t5277, t5278, t5279, t5284)
}
