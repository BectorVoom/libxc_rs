//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 942/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk942(t13945: f64, t3785: f64, t1411: f64, t1286: f64, t3732: f64, t1450: f64, t1415: f64, t10471: f64, t1337: f64, t140: f64, t1343: f64, t3480: f64, t3737: f64) -> (f64, f64, f64, f64, f64) {
    let t13946 = t3785 * t13945;
    let t13947 = t1411 * t13946;
    let t13949 = t3732 * t1286;
    let t13950 = t1450 * t13949;
    let t13951 = t1415 * t13950;
    let t13952 = t1411 * t13951;
    let t13955 = t140 * t10471 * t1337;
    let t13956 = t13955 * t1343;
    let t13959 = t140 * t3737 * t3480;
    (t13947, t13949, t13952, t13956, t13959)
}
