//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta925 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3147;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta925(t12640: f64, t488: f64, t17588: f64, t3172: f64, t3711: f64, t1261: f64, t17699: f64, t17720: f64, t3647: f64, t12904: f64, t5274: f64, t12959: f64, t17505: f64, t17225: f64, t11262: f64, t5303: f64, t5298: f64, t127: f64, t17352: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56707, t56713, t56718, t56720, t56726, t56728) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3147(t12640, t488, t17588, t3172, t3711, t1261, t17699, t17720, t3647, t12904, t5274, t12959, t17505);
        let (t56734, t56739, t56742, t56756) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3148(t17225, t3647, t11262, t1261, t5303, t3711, t5298, t127, t17352);
    (t56707, t56713, t56718, t56720, t56726, t56728, t56734, t56739, t56742, t56756)
}
