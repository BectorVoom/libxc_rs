//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1301/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1301(t14430: f64, t9985: f64, t95547: f64, t95915: f64, t1130: f64, t2178: f64, t13411: f64, t3200: f64, t13416: f64, t4554: f64, t26685: f64, t95781: f64) -> (f64, f64, f64, f64) {
    let t95921 = t14430 * t9985;
    let t95923 = t95921 * t95915 * t95547;
    let t95926 = t2178 * t1130;
    let t95928 = t3200 * t95926 * t13411;
    let t95931 = t4554 * t95926 * t13416;
    let t95938 = 0.20612155671296296296e-4_f64 * t26685 * t95781;
    (t95923, t95928, t95931, t95938)
}
