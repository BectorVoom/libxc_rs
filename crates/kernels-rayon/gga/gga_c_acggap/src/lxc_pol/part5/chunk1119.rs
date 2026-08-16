//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1119/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1119(t11878: f64, t224: f64, t6008: f64, t15043: f64, t11889: f64, t1711: f64, t709: f64, t712: f64, t11834: f64, t11837: f64, t11856: f64, t11874: f64, t20007: f64, t20008: f64, t20009: f64, t20010: f64, t20011: f64, t20013: f64, t20016: f64, t20018: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20019 = 160.0_f64 * t11878;
    let t20020 = t224 * t6008;
    let t20021 = 8.0_f64 * t20020;
    let t20022 = 0.69263436422725855034e2_f64 * t15043;
    let t20023 = 8.0_f64 * t11889;
    let t20024 = t709 * t1711;
    let t20025 = 20.0_f64 * t20024;
    let t20026 = t712 * t1711;
    let t20027 = 12.0_f64 * t20026;
    let t20028 = -t11834 + t11837 + t20007 - t20008 - t20009 - t20010 + t20011 + t11856 + t20013 - t20016 + t11874 + t20018 - t20019 + t20021 - t20022 - t20023 + t20025 + t20027;
    (t20019, t20021, t20022, t20023, t20025, t20027, t20028)
}
