//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 569/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk569(t1856: f64, t3622: f64, t1267: f64, t1846: f64, t3500: f64, t1251: f64, t2888: f64, t421: f64) -> (f64, f64, f64, f64, f64) {
    let t5281 = t1856 * t3622;
    let t5282 = t5281 * t1267;
    let t5299 = t3500 * t1846;
    let t5300 = t1251 * t5299;
    let t5302 = t2888 * t421;
    (t5281, t5282, t5299, t5300, t5302)
}
