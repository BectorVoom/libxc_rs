//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 814/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk814(t1684: f64, t3031: f64, t1823: f64, t3549: f64, t110: f64, t1852: f64, t1251: f64, t3490: f64, t5321: f64, t25: f64, t5337: f64, t11081: f64, t5325: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15450 = t1684 * t3031;
    let t15460 = t1823 * t3549;
    let t15476 = t110 * t1852;
    let t15477 = t1251 * t15476;
    let t15493 = t3490 * t5321 / 108.0_f64;
    let t15494 = t25 * t5337;
    let t15496 = t1251 * t15494 / 288.0_f64;
    let t15516 = t11081 * t5325;
    (t15450, t15460, t15477, t15493, t15496, t15516)
}
