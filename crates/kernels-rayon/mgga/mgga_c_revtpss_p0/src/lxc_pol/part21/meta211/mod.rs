//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta211(t1149: f64, t1733: f64, t3384: f64, t1723: f64, t3390: f64, t1134: f64, t3358: f64, t3394: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64) -> (f64, f64, f64, f64, f64) {
        let (t5068, t5070, t5071, t5072, t5079) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1278(t1149, t1733, t3384, t1723, t3390, t1134, t3358, t3394, t5044, t5049, t5054, t5058);
    (t5068, t5070, t5071, t5072, t5079)
}
