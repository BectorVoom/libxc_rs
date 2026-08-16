//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2866/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2866(t43813: f64, t12984: f64, t3667: f64, t1261: f64, t12879: f64, t247: f64, t3372: f64, t3368: f64, t12881: f64, t3647: f64, t1224: f64, t12268: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44865 = 0.15365432098765432099e0_f64 * t43813;
    let t44884 = t3667 * t12984;
    let t44902 = t1261 * t247 * t12879 * t3372;
    let t44906 = t1261 * t247 * t12879 * t3368;
    let t44917 = t3647 * t12881;
    let t44919 = t1224 * t12268;
    (t44865, t44884, t44902, t44906, t44917, t44919)
}
