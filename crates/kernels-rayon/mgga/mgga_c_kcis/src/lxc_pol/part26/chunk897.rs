//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 897/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk897(t1330: f64, t21020: f64, t26: f64, t659: f64, t6979: f64, t6982: f64, t21110: f64, t542: f64, t4620: f64) -> (f64, f64, f64, f64) {
    let t21198 = t1330 * t21020;
    let t21199 = t26 * t21198;
    let t21201 = t659 * t6979;
    let t21203 = t659 * t6982;
    let t21205 = t542 * t21110;
    let t21206 = t4620 * t21205;
    (t21199, t21201, t21203, t21206)
}
