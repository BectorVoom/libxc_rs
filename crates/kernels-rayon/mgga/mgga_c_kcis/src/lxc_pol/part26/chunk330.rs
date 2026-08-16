//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 330/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk330(t1386: f64, t1943: f64, t1396: f64, t1889: f64, t1395: f64, t1394: f64, t1650: f64, t518: f64) -> (f64, f64, f64, f64, f64) {
    let t1944 = t1943 * t1386;
    let t1947 = t1396 * t1889;
    let t1948 = t1395 * t1947;
    let t1949 = t1394 * t1948;
    let t1951 = t518 * t1650;
    (t1944, t1947, t1948, t1949, t1951)
}
