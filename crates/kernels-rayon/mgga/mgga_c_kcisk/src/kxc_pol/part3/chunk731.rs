//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 731/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk731(t1417: f64, t4654: f64, t1889: f64, t3517: f64, t10660: f64, t1882: f64, t706: f64, t1884: f64, t10671: f64, t677: f64, t1821: f64, t4663: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11342 = t1417 * t4654;
    let t11344 = t3517 * t1889;
    let t11346 = t1882 * t10660;
    let t11347 = t706 * t11346;
    let t11350 = t3517 * t1884;
    let t11352 = t10671 * t677;
    let t11355 = t4663 * t1821;
    (t11342, t11344, t11346, t11347, t11350, t11352, t11355)
}
