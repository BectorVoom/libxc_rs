//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1119/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1119<F: Float>(t11878: F, t224: F, t6008: F, t15043: F, t11889: F, t1711: F, t709: F, t712: F, t11834: F, t11837: F, t11856: F, t11874: F, t20007: F, t20008: F, t20009: F, t20010: F, t20011: F, t20013: F, t20016: F, t20018: F) -> (F, F, F, F, F, F, F) {
    let t20019 = F::new(160.0) * t11878;
    let t20020 = t224 * t6008;
    let t20021 = F::new(8.0) * t20020;
    let t20022 = F::cast_from(0.69263436422725855034e2_f64) * t15043;
    let t20023 = F::new(8.0) * t11889;
    let t20024 = t709 * t1711;
    let t20025 = F::new(20.0) * t20024;
    let t20026 = t712 * t1711;
    let t20027 = F::new(12.0) * t20026;
    let t20028 = -t11834 + t11837 + t20007 - t20008 - t20009 - t20010 + t20011 + t11856 + t20013 - t20016 + t11874 + t20018 - t20019 + t20021 - t20022 - t20023 + t20025 + t20027;
    (t20019, t20021, t20022, t20023, t20025, t20027, t20028)
}
