//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1237/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1237(t1267: f64, t26975: f64, t5329: f64, t6842: f64, t1020: f64, t4801: f64, t95664: f64, t2861: f64, t28992: f64, t19741: f64, t7718: f64, t18509: f64) -> (f64, f64, f64, f64, f64) {
    let t100170 = t5329 * t26975 * t6842 * t1267;
    let t100174 = t1020 * t95664 * t4801;
    let t100179 = t2861 * t28992;
    let t100188 = t1020 * t7718 * t19741;
    let t100191 = t1020 * t7718 * t18509;
    (t100170, t100174, t100179, t100188, t100191)
}
