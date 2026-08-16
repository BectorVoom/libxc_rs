//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1288/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1288(t1008: f64, t14554: f64, t167: f64, t27819: f64, t1020: f64, t13284: f64, t26760: f64, t13288: f64, t2842: f64, t1092: f64, t1121: f64, t27763: f64, t5042: f64) -> (f64, f64, f64, f64) {
    let t95721 = t14554 * t27819 * t167 * t1008;
    let t95727 = t1020 * t26760 * t13284;
    let t95730 = t2842 * t26760 * t13288;
    let t95736 = t1092 * t27763 * t5042 * t1121;
    (t95721, t95727, t95730, t95736)
}
