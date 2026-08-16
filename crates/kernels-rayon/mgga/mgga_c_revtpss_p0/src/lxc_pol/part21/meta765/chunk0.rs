//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2714/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2714(t39774: f64, t15071: f64, t892: f64, t14330: f64, t14389: f64, t2251: f64, t14322: f64, t2516: f64, t39779: f64, t2496: f64, t14426: f64, t177: f64, t762: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49945 = 0.17544670867903938621e1_f64 * t39774;
    let t49950 = t15071 * t892;
    let t49956 = 72.0_f64 * t14330 * t14389 * t2251;
    let t49957 = t14322 * t2516;
    let t49958 = 0.17544670867903938621e1_f64 * t49957;
    let t49959 = 3.0_f64 * t39779;
    let t49963 = t14322 * t2496;
    let t49964 = 0.51947577317044391276e2_f64 * t49963;
    let t49966 = t14426 * t177 * t762;
    (t49945, t49950, t49956, t49958, t49959, t49964, t49966)
}
