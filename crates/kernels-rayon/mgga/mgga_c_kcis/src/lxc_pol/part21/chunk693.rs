//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 693/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk693(t441: f64, t914: f64, t1295: f64, t235: f64, t2209: f64, t915: f64, t1250: f64, t3049: f64) -> (f64, f64, f64, f64) {
    let t7676 = t914 * t441;
    let t7679 = t235 * t1295;
    let t7684 = t915 * t2209;
    let t7687 = t3049 * t1250;
    (t7676, t7679, t7684, t7687)
}
