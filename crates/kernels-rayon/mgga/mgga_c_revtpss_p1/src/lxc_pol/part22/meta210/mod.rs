//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1336;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta210(t1151: f64, t5063: f64, t1733: f64, t3379: f64, t1149: f64, t3384: f64, t1723: f64, t3390: f64, t1134: f64, t3358: f64, t3394: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t1132: f64, t3407: f64, t1139: f64, t1729: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5065, t5067, t5068, t5070, t5071, t5072, t5079) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1336(t1151, t5063, t1733, t3379, t1149, t3384, t1723, t3390, t1134, t3358, t3394, t5044, t5049, t5054, t5058);
        let (t5080, t5087, t5088, t5090, t5093) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1337(t1132, t5079, t1723, t3407, t1134, t1139, t1729, t698);
    (t5065, t5067, t5068, t5070, t5071, t5072, t5079, t5080, t5087, t5088, t5090, t5093)
}
