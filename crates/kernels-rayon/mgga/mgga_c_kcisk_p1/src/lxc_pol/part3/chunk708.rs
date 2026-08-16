//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 708/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk708(t1701: f64, t4908: f64, t4907: f64, t617: f64, t608: f64, t10926: f64, t4911: f64, t1248: f64, t4644: f64, t4889: f64, t10488: f64, t4893: f64) -> (f64, f64, f64, f64, f64) {
    let t10978 = t1701 * t4908;
    let t10982 = 1.0_f64 / t4907 / t617;
    let t10983 = t608 * t10982;
    let t10984 = t10926 * t4911;
    let t10988 = t1248 * t4889 * t4644;
    let t10991 = t1248 * t4893 * t10488;
    (t10978, t10983, t10984, t10988, t10991)
}
