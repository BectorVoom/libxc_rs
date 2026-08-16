//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1292/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1292(t14443: f64, t27957: f64, t7703: f64, t2850: f64, t4781: f64, t4947: f64, t27953: f64, t9938: f64, t1092: f64, t27764: f64, t283: f64, t9531: f64) -> (f64, f64, f64, f64, f64) {
    let t95781 = t14443 * t27957;
    let t95783 = 0.15445601851851851852e-3_f64 * t7703 * t95781;
    let t95785 = t4947 * t4781 * t2850;
    let t95798 = 0.15445601851851851852e-3_f64 * t7703 * t9938 * t27953;
    let t95802 = t1092 * t9531 * t283 * t27764;
    (t95781, t95783, t95785, t95798, t95802)
}
