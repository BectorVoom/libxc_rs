//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 466/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk466(t1317: f64, t1897: f64, t1324: f64, t1330: f64, t1889: f64, t26: f64, t1322: f64, t1329: f64, t1891: f64, t1335: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1898 = t1317 * t1897;
    let t1901 = t1324 * t1897;
    let t1903 = t1330 * t1889;
    let t1904 = t26 * t1903;
    let t1906 = 0.1898925e1_f64 * t1898 - t1322 - 0.29896666666666666667e0_f64 * t1891 + 0.3071625e0_f64 * t1901 - t1329 - 0.82156666666666666667e-1_f64 * t1904;
    let t1907 = t1906 * t1335;
    (t1898, t1901, t1903, t1904, t1906, t1907)
}
