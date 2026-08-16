//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 630/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk630(t1201: f64, t1244: f64, t1247: f64, t1249: f64, t1729: f64, t1756: f64, t1758: f64, t3604: f64, t3610: f64, t3624: f64, t470: f64, t494: f64, t4964: f64, t5064: f64, t5069: f64, t5073: f64, t5076: f64, t5080: f64, t5084: f64, t5086: f64) -> f64 {
    let t5088 = t1201 * t1758 + t1244 * t5073 + t1244 * t5076 + t1244 * t5084 + t1247 * t5064 + t1249 * t1729 + t1756 * t3604 + 2.0_f64 * t3610 * t5069 - t3624 * t5080 + t470 * t5086 + t494 * t4964;
    t5088
}
