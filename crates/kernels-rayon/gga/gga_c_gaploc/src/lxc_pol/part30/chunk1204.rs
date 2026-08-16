//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1204/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1204(t1016: f64, t21438: f64, t10295: f64, t17288: f64, t17277: f64, t3366: f64, t2358: f64, t27232: f64, t10281: f64, t501: f64, t605: f64, t10629: f64, t5227: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32091 = t21438 * t1016;
    let t32093 = 12.0_f64 * t17288 * t10295;
    let t32095 = 2.0_f64 * t17277 * t3366;
    let t32099 = 4.0_f64 * t27232 * t2358;
    let t32100 = t10281 * t501;
    let t32102 = 2.0_f64 * t32100 * t605;
    let t32104 = 0.34180116578409885704e-2_f64 * t5227 * t10629;
    (t32091, t32093, t32095, t32099, t32102, t32104)
}
