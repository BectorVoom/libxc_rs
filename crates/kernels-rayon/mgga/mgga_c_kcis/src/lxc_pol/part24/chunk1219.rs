//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1219/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1219(t19947: f64, t28024: f64, t13106: f64, t1813: f64, t28050: f64, t28059: f64, t20178: f64, t7748: f64, t19950: f64, t26896: f64, t20159: f64, t283: f64, t6681: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99904 = t28024 * t19947;
    let t99906 = t13106 * t1813;
    let t99908 = t28059 * t28050;
    let t99910 = t7748 * t20178;
    let t99912 = t26896 * t19950;
    let t99914 = t26896 * t20159;
    let t99916 = t6681 * t283;
    (t99904, t99906, t99908, t99910, t99912, t99914, t99916)
}
