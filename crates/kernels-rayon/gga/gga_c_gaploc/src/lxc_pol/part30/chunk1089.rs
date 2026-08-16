//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1089/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1089(t161: f64, t23000: f64, t28126: f64, t5241: f64, t10012: f64, t1710: f64, t2684: f64, t9438: f64, t23099: f64, t7396: f64, t948: f64, t10018: f64, t7375: f64) -> (f64, f64, f64, f64) {
    let t28129 = 0.23005755572352449806e1_f64 * t23000 * t5241 * t161 * t28126;
    let t28141 = t2684 * t9438 * t10012 * t1710;
    let t28150 = t23099 * t948 * t7396;
    let t28151 = 0.76685851907841499352e0_f64 * t28150;
    let t28156 = t7375 * t10018;
    (t28129, t28141, t28151, t28156)
}
