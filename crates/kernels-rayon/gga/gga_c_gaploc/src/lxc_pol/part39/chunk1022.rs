//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1022/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1022(t326: f64, t43494: f64, t825: f64, t2684: f64, t7585: f64, t2365: f64, t32261: f64, t7390: f64, t43107: f64, t723: f64) -> (f64, f64, f64, f64) {
    let t43497 = 0.18404604457881959845e2_f64 * t825 * t326 * t43494;
    let t43500 = 0.14953741122029092374e3_f64 * t2684 * t7585 * t43494;
    let t43502 = t7390 * t2365 * t32261;
    let t43508 = t43107 * t723;
    (t43497, t43500, t43502, t43508)
}
