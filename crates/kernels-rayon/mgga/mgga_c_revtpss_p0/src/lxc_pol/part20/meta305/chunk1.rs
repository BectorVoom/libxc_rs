//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1195/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1195(t12295: f64, t12351: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t12344: f64, t12347: f64, t12354: f64) -> f64 {
    let t12542 = 0.93932222222222222223e0_f64 * t12295;
    let t12543 = 0.36793333333333333333e0_f64 * t12351;
    let t12546 = 0.20128333333333333333e0_f64 * t12299 + 0.33547222222222222222e0_f64 * t12307 + 0.40256666666666666668e0_f64 * t12297 - 0.60385000000000000001e0_f64 * t12301 - 0.30192500000000000001e0_f64 * t12303 - 0.12077e1_f64 * t12310 + 0.181155e1_f64 * t12314 + 0.301925e0_f64 * t12320 - 0.3883875e1_f64 * t12344 + 0.247573125e0_f64 * t12347 - t12542 - t12543 + 0.181155e1_f64 * t12317 + 0.16504875e0_f64 * t12354;
    t12546
}
