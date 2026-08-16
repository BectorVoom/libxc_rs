//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 612/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk612(t4160: f64, t6905: f64, t1889: f64, t5632: f64, t1395: f64, t1394: f64, t1444: f64, t6281: f64) -> (f64, f64, f64, f64, f64) {
    let t6906 = t4160 * t6905;
    let t6908 = t5632 * t1889;
    let t6909 = t1395 * t6908;
    let t6910 = t1394 * t6909;
    let t6912 = t1444 * t6281;
    (t6906, t6908, t6909, t6910, t6912)
}
