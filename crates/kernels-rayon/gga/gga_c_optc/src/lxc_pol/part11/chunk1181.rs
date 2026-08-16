//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1181/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1181(t15104: f64, t17623: f64, t18198: f64, t5236: f64, t5238: f64, t8: f64, t15063: f64, t17622: f64, t17627: f64, t43671: f64, t11782: f64, t18213: f64) -> (f64, f64, f64, f64, f64) {
    let t53776 = t17623 * t15104;
    let t53793 = t5236 * t5238 * t18198 * t8;
    let t53812 = t17622 * t15063;
    let t53823 = t43671 * t17627;
    let t53825 = t11782 * t18213;
    (t53776, t53793, t53812, t53823, t53825)
}
