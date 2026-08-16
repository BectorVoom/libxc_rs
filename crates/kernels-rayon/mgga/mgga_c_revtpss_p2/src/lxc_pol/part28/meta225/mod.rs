//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1061;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta225(t3357: f64, t3358: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t422: f64, t1130: f64, t1719: f64, t1151: f64, t1733: f64, t3379: f64, t1149: f64, t3384: f64, t1723: f64, t3390: f64, t1134: f64, t3394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5060, t5062, t5063, t5065, t5067) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1061(t3357, t3358, t5044, t5049, t5054, t5058, t422, t1130, t1719, t1151, t1733, t3379);
        let (t5068, t5070, t5071, t5072, t5079) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1062(t1149, t1733, t3384, t1723, t3390, t1134, t3358, t3394, t5044, t5049, t5054, t5058);
    (t5060, t5062, t5063, t5065, t5067, t5068, t5070, t5071, t5072, t5079)
}
