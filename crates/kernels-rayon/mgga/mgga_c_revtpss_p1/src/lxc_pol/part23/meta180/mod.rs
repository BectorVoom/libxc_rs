//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta180 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta180(t3417: f64, t5047: f64, t141: f64, t1145: f64, t5052: f64, t5056: f64, t3358: f64, t3402: f64, t3414: f64, t3415: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t5072: f64, t5080: f64, t5088: f64, t5090: f64, t5093: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5095, t5096, t5098, t5099, t5101, t5102, t5104) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1077(t3417, t5047, t141, t1145, t5052, t5056, t3358, t3402, t3414, t3415, t5044, t5049, t5054, t5058, t5072, t5080, t5088, t5090, t5093);
    (t5095, t5096, t5098, t5099, t5101, t5102, t5104)
}
