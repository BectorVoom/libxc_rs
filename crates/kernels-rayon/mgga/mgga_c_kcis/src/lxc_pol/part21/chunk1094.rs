//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1094/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1094(t1021: f64, t3358: f64, t3355: f64, t3348: f64, t26889: f64, t26892: f64, t26894: f64, t26898: f64, t26900: f64, t26902: f64, t26904: f64, t26906: f64, t26908: f64) -> (f64, f64, f64, f64) {
    let t26910 = t1021 * t3358;
    let t26912 = t1021 * t3355;
    let t26914 = t1021 * t3348;
    let t26916 = -t26889 / 64.0_f64 + t26892 / 3.0_f64 - t26894 / 12.0_f64 + t26898 / 8.0_f64 - t26900 / 96.0_f64 + t26902 / 128.0_f64 + t26904 / 12.0_f64 - t26906 / 48.0_f64 + t26908 / 64.0_f64 + t26910 / 9.0_f64 - 19.0_f64 / 72.0_f64 * t26912 - t26914 / 288.0_f64;
    (t26910, t26912, t26914, t26916)
}
