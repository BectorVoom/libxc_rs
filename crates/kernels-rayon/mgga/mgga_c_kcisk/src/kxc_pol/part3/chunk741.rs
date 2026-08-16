//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 741/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk741(t11450: f64, t1801: f64, t1873: f64, t1869: f64, t10375: f64, t1900: f64, t213: f64, t568: f64, t682: f64, t1810: f64, t1846: f64, t1825: f64, t5082: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11451 = t1801 * t11450;
    let t11452 = t1873 * t11451;
    let t11453 = t1869 * t11452;
    let t11455 = t10375 * t1900;
    let t11456 = t1869 * t11455;
    let t11458 = t213 * t568;
    let t11460 = 0.14055920378328537299e-1_f64 * t11458 * t682;
    let t11461 = t1846 * t1810;
    let t11463 = t5082 * t1825;
    (t11453, t11456, t11458, t11460, t11461, t11463)
}
