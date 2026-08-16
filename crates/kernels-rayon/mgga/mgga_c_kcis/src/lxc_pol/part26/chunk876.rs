//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 876/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk876(t20984: f64, t5662: f64, t4170: f64, t4160: f64, t5627: f64, t5632: f64, t1468: f64, t1464: f64, t1889: f64, t5676: f64, t15887: f64, t5880: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20985 = t5662 * t20984;
    let t20986 = t4170 * t20985;
    let t20987 = t4160 * t20986;
    let t20989 = t5632 * t5627;
    let t20990 = t1468 * t20989;
    let t20991 = t1464 * t20990;
    let t20994 = t1889 * t5676;
    let t20995 = t15887 * t20994;
    let t20996 = t4160 * t20995;
    let t20998 = t1889 * t5880;
    (t20985, t20987, t20989, t20991, t20994, t20996, t20998)
}
